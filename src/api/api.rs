use std::error::Error;
use jwt::{Header, RegisteredClaims, Token};
use tonic::{Request, Response, Status};
use tonic::metadata::MetadataMap;

use crate::{api, domain};
use crate::api::ApiService;
use crate::api::api_v1_server::{ApiV1};

use crate::domain::media::aggregates::ImageId;
use crate::domain::club::aggregates::{Club, ClubId, ClubName};
use crate::domain::club::commands::{AddStaffMember, New, RemoveStaffMember, SetLogo};
use crate::domain::social::aggregates::{Comment, CommentId, CommentText, Community, CommunityContext, CommunityId, CommunityName, Feed, FeedListing, Post, PostAttachment, PostAttachments, PostId, PostReaction, PostText};
use crate::domain::team::aggregates::{Team, TeamId, TeamName};
use crate::domain::account::aggregates::UserId;

#[tonic::async_trait]
impl ApiV1 for ApiService {
    // queries
    async fn list_clubs(&self, request: Request<api::ListClubsRequest>) -> Result<Response<api::ListClubsResponse>, Status> {
        let payload = request.into_inner();
        let after = if !payload.after.is_empty() {
            ClubId::parse(payload.after.as_str())
                .map_err(|_| Status::unknown("malformed after value"))
                .map(|r| Some(r))
            } else {
                Ok(None)
            }?;

        self.club_usecase.list_clubs(after)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::ListClubsResponse {
                    clubs: result.iter().map(to_club).collect(),
                })
            )
    }

    async fn list_teams(&self, request: Request<api::ListTeamsRequest>) -> Result<Response<api::ListTeamsResponse>, Status> {
        let payload = request.into_inner();
        let after = if !payload.after.is_empty() {
            TeamId::parse(payload.after.as_str())
                .map_err(|_| Status::unknown("malformed after value"))
                .map(|r| Some(r))
        } else {
            Ok(None)
        }?;

        self.team_usecase.list_teams(after)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::ListTeamsResponse {
                    teams: result.iter().map(to_team).collect(),
                })
            )
    }

    async fn list_communities(&self, request: Request<api::ListCommunitiesRequest>) -> Result<Response<api::ListCommunitiesResponse>, Status> {
        let payload = request.into_inner();

        let context = match payload.context {
            Some(api::list_communities_request::Context::ClubId(id)) => {
                ClubId::parse(id.as_str())
                    .map_err(|_| Status::unknown("malformed context value"))
                    .map(|club| Some(CommunityContext::Club(club)))?
            },
            Some(api::list_communities_request::Context::TeamId(id)) => {
                TeamId::parse(id.as_str())
                    .map_err(|_| Status::unknown("malformed context value"))
                    .map(|team| Some(CommunityContext::Team(team)))?
            },
            _ => None
        };

        let after = if !payload.after.is_empty() {
            CommunityId::parse(payload.after.as_str())
                .map_err(|_| Status::unknown("malformed after value"))
                .map(|r| Some(r))
        } else {
            Ok(None)
        }?;

        self.social_usecase.list_communities(context, after)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::ListCommunitiesResponse {
                    communities: result.iter().map(to_community).collect(),
                })
            )
    }

    async fn list_comments(&self, request: Request<api::ListCommentsRequest>) -> Result<Response<api::ListCommentsResponse>, Status> {
        let payload = request.into_inner();
        let reply_to = PostId::parse(payload.reply_to_id.as_str())
            .map_err(|_| Status::unknown("malformed reply_to_id value"))?;
        let after = if !payload.after.is_empty() {
            CommentId::parse(payload.after.as_str())
                .map_err(|_| Status::unknown("malformed after value"))
                .map(|r| Some(r))
        } else {
            Ok(None)
        }?;

        self.social_usecase.list_comments(reply_to, after)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::ListCommentsResponse {
                    comments: result.iter().map(to_comment).collect(),
                })
            )
    }

    async fn list_feed(&self, request: Request<api::ListFeedRequest>) -> Result<Response<api::ListFeedResponse>, Status> {
        let person = current_user(request.metadata())?;
        let payload = request.into_inner();
        let feed = match payload.feed {
            Some(api::list_feed_request::Feed::Memberships(_)) => {
                Ok(Feed::Memberships(person))
            },
            Some(api::list_feed_request::Feed::CommunityId(id)) => {
                CommunityId::parse(id.as_str())
                    .map_err(|_| Status::unknown("malformed community value"))
                    .map(|community| Feed::Community(community))
            },
            _ =>
                Err(Status::unknown("malformed feed"))
        }?;
        let after = if !payload.after.is_empty() {
            PostId::parse(payload.after.as_str())
                .map_err(|_| Status::unknown("malformed after value"))
                .map(|r| Some(r))
        } else {
            Ok(None)
        }?;

        self.social_usecase.list_feed(feed, after)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::ListFeedResponse {
                    listings: result.iter().map(to_feed_listing).collect(),
                })
            )
    }

    // commands
    // - club
    async fn new_club(&self, request: Request<api::NewClubRequest>) -> Result<Response<api::NewClubResponse>, Status> {
        let name = ClubName::parse(request.into_inner().name.as_str())
            .map_err(|_| Status::unknown("malformed name value"))?;

        let command = New {
            name
        };

        self.club_usecase.new(command)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::NewClubResponse {
                    id: result.id.to_string(),
                })
            )
    }

    async fn set_club_logo(&self, request: Request<api::SetClubLogoRequest>) -> Result<Response<api::SetClubLogoResponse>, Status> {
        let payload = request.into_inner();
        let club = ClubId::parse(payload.club_id.as_str())
            .map_err(|_| Status::unknown("malformed club_id value"))?;
        let logo = ImageId::parse(payload.logo_id.as_str())
            .map_err(|_| Status::unknown("malformed logo_id value"))?;

        let command = SetLogo {
            club,
            logo,
        };

        self.club_usecase.set_logo(command).await
            .map_err(to_status)
            .map(|_|
                Response::new(api::SetClubLogoResponse {})
            )
    }

    async fn add_staff_member_to_club(&self, request: Request<api::AddStaffMemberToClubRequest>) -> Result<Response<api::AddStaffMemberToClubResponse>, Status> {
        let payload = request.into_inner();
        let club = ClubId::parse(payload.club_id.as_str())
            .map_err(|_| Status::unknown("malformed club_id value"))?;
        let person = UserId::parse(payload.person_id.as_str())
            .map_err(|_| Status::unknown("malformed person_id value"))?;

        let command = AddStaffMember {
            club,
            person,
        };

        self.club_usecase.add_staff_member(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::AddStaffMemberToClubResponse {})
            )
    }

    async fn remove_staff_member_from_club(&self, request: Request<api::RemoveStaffMemberFromClubRequest>) -> Result<Response<api::RemoveStaffMemberFromClubResponse>, Status> {
        let payload = request.into_inner();
        let club = ClubId::parse(payload.club_id.as_str())
            .map_err(|_| Status::unknown("malformed club_id value"))?;
        let staff_member = UserId::parse(payload.staff_member_id.as_str())
            .map_err(|_| Status::unknown("malformed staff_member_id value"))?;

        let command = RemoveStaffMember {
            club,
            staff_member,
        };

        self.club_usecase.remove_staff_member(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::RemoveStaffMemberFromClubResponse {})
            )
    }

    // - team
    async fn new_team(&self, request: Request<api::NewTeamRequest>) -> Result<Response<api::NewTeamResponse>, Status> {
        let payload = request.into_inner();
        let name = TeamName::parse(payload.name.as_str())
            .map_err(|_| Status::unknown("malformed name value"))?;
        let club = ClubId::parse(payload.club_id.as_str())
            .map_err(|_| Status::unknown("malformed club_id value"))?;

        let command = crate::domain::team::commands::New {
            name,
            club,
        };

        self.team_usecase.new(command)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::NewTeamResponse {
                    id: result.id.to_string(),
                })
            )
    }

    async fn add_staff_member_to_team(&self, request: Request<api::AddStaffMemberToTeamRequest>) -> Result<Response<api::AddStaffMemberToTeamResponse>, Status> {
        let payload = request.into_inner();
        let team = TeamId::parse(payload.team_id.as_str())
            .map_err(|_| Status::unknown("malformed team_id value"))?;
        let person = UserId::parse(payload.person_id.as_str())
            .map_err(|_| Status::unknown("malformed person_id value"))?;

        let command = crate::domain::team::commands::AddStaffMember {
            team,
            person,
        };

        self.team_usecase.add_staff_member(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::AddStaffMemberToTeamResponse {})
            )
    }

    async fn remove_staff_member_from_team(&self, request: Request<api::RemoveStaffMemberFromTeamRequest>) -> Result<Response<api::RemoveStaffMemberFromTeamResponse>, Status> {
        let payload = request.into_inner();
        let team = TeamId::parse(payload.team_id.as_str())
            .map_err(|_| Status::unknown("malformed team_id value"))?;
        let staff_member = UserId::parse(payload.staff_member_id.as_str())
            .map_err(|_| Status::unknown("malformed staff_member_id value"))?;

        let command = crate::domain::team::commands::RemoveStaffMember {
            team,
            staff_member,
        };

        self.team_usecase.remove_staff_member(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::RemoveStaffMemberFromTeamResponse {})
            )
    }


    // - community
    async fn new_community(&self, request: Request<api::NewCommunityRequest>) -> Result<Response<api::NewCommunityResponse>, Status> {
        let payload = request.into_inner();
        let name = CommunityName::parse(payload.name.as_str())
            .map_err(|_| Status::unknown("malformed name value"))?;
        let context = match payload.context {
            Some(api::new_community_request::Context::ClubId(id)) => {
                let club = ClubId::parse(id.as_str())
                    .map_err(|_| Status::unknown("malformed context value"))?;

                Ok(CommunityContext::Club(club))
            },
            Some(api::new_community_request::Context::TeamId(id)) => {
                let team = TeamId::parse(id.as_str())
                    .map_err(|_| Status::unknown("malformed context value"))?;

                Ok(CommunityContext::Team(team))
            },
            _ =>
              Err(Status::unknown("malformed context value"))
        }?;

        let command = crate::domain::social::commands::community::New {
            name,
            context,
        };

        self.social_usecase.new(command)
            .await
            .map_err(to_status)
            .map(|result|
                 Response::new(api::NewCommunityResponse {
                     id: result.id.to_string(),
                 })
            )
    }

    async fn set_community_logo(&self, request: Request<api::SetCommunityLogoRequest>) -> Result<Response<api::SetCommunityLogoResponse>, Status> {
        let payload = request.into_inner();
        let community = CommunityId::parse(payload.community_id.as_str())
            .map_err(|_| Status::unknown("malformed community_id value"))?;
        let logo = ImageId::parse(payload.logo_id.as_str())
            .map_err(|_| Status::unknown("malformed logo_id value"))?;

        let command = domain::social::commands::community::SetLogo {
            community,
            logo,
        };

        self.social_usecase.set_logo(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::SetCommunityLogoResponse {})
            )
    }

    async fn promote_community_member_to_editor(&self, request: Request<api::PromoteCommunityMemberToEditorRequest>) -> Result<Response<api::PromoteCommunityMemberToEditorResponse>, Status> {
        let payload = request.into_inner();
        let community = CommunityId::parse(payload.community_id.as_str())
            .map_err(|_| Status::unknown("malformed community_id value"))?;
        let member = UserId::parse(payload.member_id.as_str())
            .map_err(|_| Status::unknown("malformed member_id value"))?;

        let command = domain::social::commands::community::PromoteMemberToEditor {
            community,
            member,
        };

        self.social_usecase.promote_member_to_editor(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::PromoteCommunityMemberToEditorResponse {})
            )
    }

    async fn demote_community_editor(&self, request: Request<api::DemoteCommunityEditorRequest>) -> Result<Response<api::DemoteCommunityEditorResponse>, Status> {
        let payload = request.into_inner();
        let community = CommunityId::parse(payload.community_id.as_str())
            .map_err(|_| Status::unknown("malformed community_id value"))?;
        let editor = UserId::parse(payload.editor_id.as_str())
            .map_err(|_| Status::unknown("malformed editor_id value"))?;

        let command = domain::social::commands::community::DemoteEditor {
            community,
            editor,
        };

        self.social_usecase.demote_editor(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::DemoteCommunityEditorResponse {})
            )
    }

    async fn join_community(&self, request: Request<api::JoinCommunityRequest>) -> Result<Response<api::JoinCommunityResponse>, Status> {
        let person = current_user(request.metadata())?;
        let payload = request.into_inner();
        let community = CommunityId::parse(payload.community_id.as_str())
            .map_err(|_| Status::unknown("malformed community_id value"))?;

        let command = domain::social::commands::community::Join {
            community,
            person,
        };

        self.social_usecase.join(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::JoinCommunityResponse {})
            )
    }

    async fn leave_community(&self, request: Request<api::LeaveCommunityRequest>) -> Result<Response<api::LeaveCommunityResponse>, Status> {
        let member = current_user(request.metadata())?;
        let payload = request.into_inner();
        let community = CommunityId::parse(payload.community_id.as_str())
            .map_err(|_| Status::unknown("malformed community_id value"))?;

        let command = domain::social::commands::community::Leave {
            community,
            member,
        };

        self.social_usecase.leave(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::LeaveCommunityResponse {})
            )
    }

    // - post
    async fn publish_post(&self, request: Request<api::PublishPostRequest>) -> Result<Response<api::PublishPostResponse>, Status> {
        let author = current_user(request.metadata())?;
        let payload = request.into_inner();
        let text = PostText::parse(payload.text.as_str())
            .map_err(|_| Status::unknown("malformed text value"))?;
        let community = CommunityId::parse(payload.community_id.as_str())
            .map_err(|_| Status::unknown("malformed community_id value"))?;
        let attachment_elements: Result<Vec<PostAttachment>, String> = payload.attachments
            .into_iter().map(parse_attachment).collect();
        let attachments = PostAttachments::from_vec(attachment_elements
            .map_err(|err| Status::unknown(err))?);

        let command = domain::social::commands::post::PublishPost {
            community,
            text,
            attachments,
            author
        };

        self.social_usecase.publish_post(command)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::PublishPostResponse {
                    id: result.id.to_string(),
                })
            )
    }

    async fn remove_post(&self, request: Request<api::RemovePostRequest>) -> Result<Response<api::RemovePostResponse>, Status> {
        let payload = request.into_inner();
        let post = PostId::parse(payload.post_id.as_str())
            .map_err(|_| Status::unknown("malformed post_id value"))?;

        let command = domain::social::commands::post::RemovePost {
            post,
        };

        self.social_usecase.remove_post(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::RemovePostResponse {})
            )
    }

    // - post reaction
    async fn react_to_post(&self, request: Request<api::ReactToPostRequest>) -> Result<Response<api::ReactToPostResponse>, Status> {
        let author = current_user(request.metadata())?;
        let payload = request.into_inner();
        let post = PostId::parse(payload.post_id.as_str())
            .map_err(|_| Status::unknown("malformed post_id value"))?;
        let reaction = parse_post_reaction(payload.emotion, post, author)
            .ok_or(Status::unknown("unsupported emotion"))?;

        let command = domain::social::commands::post_reaction::ReactToPost {
            reaction,
        };

        self.social_usecase.react_to_post(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::ReactToPostResponse{})
            )
    }

    async fn retract_post_reaction(&self, request: Request<api::RetractPostReactionRequest>) -> Result<Response<api::RetractPostReactionResponse>, Status> {
        let author = current_user(request.metadata())?;
        let payload = request.into_inner();
        let post = PostId::parse(payload.post_id.as_str())
            .map_err(|_| Status::unknown("malformed post_id value"))?;
        let reaction = parse_post_reaction(payload.emotion, post, author)
            .ok_or(Status::unknown("unsupported emotion"))?;

        let command = domain::social::commands::post_reaction::RetractPostReaction {
            reaction,
        };

        self.social_usecase.retract_postreaction(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::RetractPostReactionResponse{})
            )
    }

    // - comment
    async fn publish_comment(&self, request: Request<api::PublishCommentRequest>) -> Result<Response<api::PublishCommentResponse>, Status> {
        let author = current_user(request.metadata())?;
        let payload = request.into_inner();
        let reply_to = PostId::parse(payload.reply_to_id.as_str())
            .map_err(|_| Status::unknown("malformed reply_to_id value"))?;
        let text = CommentText::parse(payload.text.as_str())
            .map_err(|_| Status::unknown("malformed text value"))?;

        let command = domain::social::commands::comment::PublishComment {
            reply_to,
            text,
            author
        };

        self.social_usecase.publish_comment(command)
            .await
            .map_err(to_status)
            .map(|result|
                Response::new(api::PublishCommentResponse {
                    id: result.id.to_string(),
                })
            )
    }

    async fn remove_comment(&self, request: Request<api::RemoveCommentRequest>) -> Result<Response<api::RemoveCommentResponse>, Status> {
        let payload = request.into_inner();
        let comment = CommentId::parse(payload.comment_id.as_str())
            .map_err(|_| Status::unknown("malformed comment_id value"))?;

        let command = domain::social::commands::comment::RemoveComment {
            comment,
        };

        self.social_usecase.remove_comment(command)
            .await
            .map_err(to_status)
            .map(|_|
                Response::new(api::RemoveCommentResponse {})
            )
    }
}

// helpers
fn to_status(error: Box<dyn Error + Send + Sync>) -> Status {
    Status::unknown(error.to_string())
}

fn current_user(metadata: &MetadataMap) -> Result<UserId, Status> {
    let error = || Status::unauthenticated("corrucpt authorization data");

    let header = metadata
        .get("authorization").ok_or(error())?
        .to_str().map_err(|_| error())?
        .strip_prefix("Bearer ").ok_or(error())?;

    let token: Token<Header, RegisteredClaims, _> = Token::parse_unverified(header)
        .map_err(|_| error())?;

    let subject = token.claims()
        .subject.as_ref()
        .ok_or(error())?;

    UserId::parse(&subject).map_err(|_| error())
}

// parse + to transfer objects
fn parse_post_reaction(emotion: i32, post: PostId, author: UserId) -> Option<PostReaction> {
    if emotion == api::Emotion::Love as i32 {
        Some(PostReaction::Love(author, post))
    } else if emotion == api::Emotion::Funny as i32 {
        Some(PostReaction::Funny(author, post))
    } else if emotion == api::Emotion::Celebrate as i32 {
        Some(PostReaction::Celebrate(author, post))
    } else if emotion == api::Emotion::Support as i32 {
        Some(PostReaction::Support(author, post))
    }  else if emotion == api::Emotion::Insightful as i32 {
        Some(PostReaction::Insightful(author, post))
    } else {
        None
    }
}

fn parse_attachment(input: api::publish_post_request::Attachment) -> Result<PostAttachment, String> {
    match input.media  {
        Some(api::publish_post_request::attachment::Media::ImageId(id)) =>
          ImageId::parse(&id)
              .map(|image| PostAttachment::Image(image))
              .map_err(|_| String::from("malformed (attachment) image_id value")),

        _ =>
            Err(String::from("unsupported post attachment"))
    }
}

fn to_some_logo(logo: &Option<ImageId>) -> String {
    match logo {
        Some(image) => image.to_string(),
        None => String::default(),
    }
}

fn to_club(club: &Club) -> api::Club {
    api::Club {
        id: club.id.to_string(),
        name: club.name.to_string(),
        logo_id: to_some_logo(&club.logo),
        staff_ids: club.staff.iter().map(|s| s.to_string()).collect(),
    }
}

fn to_team(team: &Team) -> api::Team {
    api::Team {
        id: team.id.to_string(),
        name: team.name.to_string(),
        club_id: team.club.to_string(),
        staff_ids: team.staff.iter().map(|s| s.to_string()).collect(),
    }
}

fn to_community(community: &Community) -> api::Community {
    let context = match &community.context {
        CommunityContext::Club(id) => Some(api::community::Context::ClubId(id.to_string())),
        CommunityContext::Team(id) => Some(api::community::Context::TeamId(id.to_string())),
    };

    api::Community {
        id: community.id.to_string(),
        name: community.name.to_string(),
        context,
        founded: community.founded.to_rfc3339(),
        logo_id: to_some_logo(&community.logo),
        editor_ids: community.editors.iter().map(|e| e.to_string()).collect(),
        member_count: u64::try_from(community.members.len()).unwrap_or(0),
    }
}

fn to_comment(comment: &Comment) -> api::Comment {
    api::Comment {
        id: comment.id.to_string(),
        text: comment.text.to_string(),
        author_id: comment.author.to_string(),
        published: comment.published.timestamp_millis() as u64,
    }
}

fn to_post_attachment(attachment: &PostAttachment) -> api::PostAttachment {
    match attachment {
        PostAttachment::Image(id) => api::PostAttachment {
            r#type: Some(api::post_attachment::Type::ImageId(id.to_string())),
        }
    }
}

fn to_post(post: &Post) -> api::Post {
    api::Post {
        id: post.id.to_string(),
        community_id: post.community.to_string(),
        text: post.text.to_string(),
        attachments: post.attachments.iter().map(to_post_attachment).collect(),
        author_id: post.author.to_string(),
        published: post.published.timestamp_millis() as u64,
    }
}

fn to_feed_listing(listing: &FeedListing) -> api::list_feed_response::FeedListing {
    api::list_feed_response::FeedListing {
        post: Some(to_post(&listing.post)),
        comments: listing.comments,
        reactions: Some(api::list_feed_response::Reactions {
            love: listing.reactions_love,
            funny: listing.reactions_funny,
            celebrate: listing.reactions_celebrate,
            support: listing.reactions_support,
            insightful: listing.reactions_insightful,
        }),
    }
}
