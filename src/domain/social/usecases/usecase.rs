use std::error::Error;
use std::sync::Arc;
use chrono::{Utc};
use crate::common::EventPublisher;
use crate::domain::social::aggregates::{Comment, CommentId, Community, CommunityContext, CommunityId, Feed, FeedFragment, Post, PostId};
use crate::domain::social::commands::comment::{PublishComment, PublishCommentResult, RemoveComment};
use crate::domain::social::commands::community::{DemoteEditor, Join, Leave, New, NewResult, PromoteMemberToEditor, SetLogo};
use crate::domain::social::commands::post::{PublishPost, PublishPostResult, RemovePost};
use crate::domain::social::commands::post_reaction::{ReactToPost, RetractPostReaction};
use crate::domain::social::events::{CommentPublishedV1, CommentRemovedV1, CommunityAddedV1, CommunityLogoSetV1, EditorDemotedV1, JoinedV1, LeftV1, MemberPromotedToEditorV1, PostPublishedV1, PostReactionRetractedV1, PostRemovedV1, ReactedToPostV1};
use crate::domain::social::repositories::{CommentRepository, CommunityRepository, FeedRepository, PostReactionRepository, PostRepository};
use crate::domain::social::usecases::error::DomainError;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct SocialUsecase {
    community_repository: Box<dyn CommunityRepository + Send + Sync>,
    post_repository: Box<dyn PostRepository + Send + Sync>,
    post_reaction_repository: Box<dyn PostReactionRepository + Send + Sync>,
    comment_repository: Box<dyn CommentRepository + Send + Sync>,
    feed_repository: Box<dyn FeedRepository + Send + Sync>,
    event_publisher: Arc<EventPublisher>,
}

impl SocialUsecase {
    pub fn build(
        community_repository: Box<dyn CommunityRepository + Send + Sync>,
        post_repository: Box<dyn PostRepository + Send + Sync>,
        post_reaction_repository: Box<dyn PostReactionRepository + Send + Sync>,
        comment_repository: Box<dyn CommentRepository + Send + Sync>,
        feed_repository: Box<dyn FeedRepository + Send + Sync>,
        event_publisher: Arc<EventPublisher>) -> SocialUsecase {
        SocialUsecase {
            community_repository,
            post_repository,
            post_reaction_repository,
            comment_repository,
            feed_repository,
            event_publisher,
        }
    }

    // commands
    // - community
    pub async fn new(&self, command: New) -> Result<NewResult> {
        let id = CommunityId::random();
        let name = command.name;
        let context = command.context;
        let founded = Utc::now();

        let community = Community::new(id.clone(), name, context, founded);
        self.community_repository.set(&community).await?;

        let event = CommunityAddedV1 {
            id: community.id,
            name: community.name,
            context: community.context,
            founded: community.founded,
        };
        self.event_publisher.publish(&event).await?;

        Ok(NewResult{
            id,
        })
    }

    pub async fn set_logo(&self, command: SetLogo) -> Result<()> {
        let mut community = self.community_repository
            .get(&command.community)
            .await?
            .ok_or(DomainError::UnknownCommunity)?;

        community.set_logo(&command.logo);
        self.community_repository.set(&community).await?;

        let event = CommunityLogoSetV1 { community: command.community, logo: command.logo };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    pub async fn promote_member_to_editor(&self, command: PromoteMemberToEditor) -> Result<()> {
        let mut community = self.community_repository
            .get(&command.community)
            .await?
            .ok_or(DomainError::UnknownCommunity)?;

        let promoted = community.promote_member_to_editor(&command.member);
        if promoted {
            self.community_repository.set(&community).await?;

            let event = MemberPromotedToEditorV1 { community: command.community, member: command.member };
            self.event_publisher.publish(&event).await?;
        }

        Ok(())
    }

    pub async fn demote_editor(&self, command: DemoteEditor) -> Result<()> {
        let mut community = self.community_repository
            .get(&command.community)
            .await?
            .ok_or(DomainError::UnknownCommunity)?;

        let demoted = community.demote_editor(&command.editor);
        if demoted {
            self.community_repository.set(&community).await?;

            let event = EditorDemotedV1 { community: command.community, editor: command.editor };
            self.event_publisher.publish(&event).await?;
        }

        Ok(())
    }

    pub async fn join(&self, command: Join) -> Result<()> {
        let mut community = self.community_repository
            .get(&command.community)
            .await?
            .ok_or(DomainError::UnknownCommunity)?;

        let joined = community.join(&command.person);
        if joined {
            self.community_repository.set(&community).await?;

            let event = JoinedV1 { community: command.community, person: command.person };
            self.event_publisher.publish(&event).await?;
        }

        Ok(())
    }

    pub async fn leave(&self, command: Leave) -> Result<()> {
        let mut community = self.community_repository
            .get(&command.community)
            .await?
            .ok_or(DomainError::UnknownCommunity)?;

        let left = community.leave(&command.member);
        if left {
            self.community_repository.set(&community).await?;

            let event = LeftV1 { community: command.community, member: command.member };
            self.event_publisher.publish(&event).await?;
        }

        Ok(())
    }

    // - post
    pub async fn publish_post(&self, command: PublishPost) -> Result<PublishPostResult> {
        let id = PostId::random();
        let community = command.community;
        let text = command.text;
        let attachments = command.attachments;
        let author = command.author;
        let published = Utc::now();

        let post = Post::new(id.clone(), community, text, attachments, author, published);
        self.post_repository.set(&post).await?;

        let event = PostPublishedV1 {
            id: post.id,
            community: post.community,
            text: post.text,
            attachments: post.attachments,
            author: post.author,
            published: post.published,
        };
        self.event_publisher.publish(&event).await?;

        Ok(PublishPostResult {
            id
        })
    }

    pub async fn remove_post(&self, command: RemovePost) -> Result<()> {
        let id = command.post;
        let _post = self.post_repository
            .get(&id)
            .await?
            .ok_or(DomainError::UnknownPost)?;

        self.post_repository.remove(&id).await?;

        let event = PostRemovedV1 {
            id
        };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    pub async fn react_to_post(&self, command: ReactToPost) -> Result<()> {
        let (_, _, post) = command.reaction.values();
        if let None = self.post_repository.get(&post).await? {
            return Err(DomainError::UnknownPost.into());
        }

        let set = self.post_reaction_repository.set(&command.reaction).await?;
        if set {
            let event = ReactedToPostV1 { reaction: command.reaction };
            self.event_publisher.publish(&event).await?;
        }

        Ok(())
    }

    pub async fn retract_postreaction(&self, command: RetractPostReaction) -> Result<()> {
        let unset = self.post_reaction_repository.unset(&command.reaction).await?;
        if unset {
            let event = PostReactionRetractedV1 { reaction: command.reaction };
            self.event_publisher.publish(&event).await?;
        }

        Ok(())
    }

    // - comment
    pub async fn publish_comment(&self, command: PublishComment) -> Result<PublishCommentResult> {
        let id = CommentId::random();
        let reply_to = command.reply_to;
        let text = command.text;
        let author = command.author;
        let published = Utc::now();

        if let None = self.post_repository.get(&reply_to).await? {
            return Err(DomainError::UnknownPost.into());
        }

        let comment = Comment::new(id.clone(), reply_to, text, author, published);
        self.comment_repository.set(&comment).await?;

        let event = CommentPublishedV1 {
            id: comment.id,
            reply_to: comment.reply_to,
            text: comment.text,
            author: comment.author,
            published: comment.published,
        };
        self.event_publisher.publish(&event).await?;

        Ok(PublishCommentResult {
            id
        })
    }

    pub async fn remove_comment(&self, command: RemoveComment) -> Result<()> {
        let id = command.comment;
        let _comment = self.comment_repository
            .get(&id)
            .await?
            .ok_or(DomainError::UnknownComment)?;

        self.comment_repository.remove(&id).await?;

        let event = CommentRemovedV1 {
            id
        };
        self.event_publisher.publish(&event).await?;

        Ok(())
    }

    // queries
    pub async fn list_communities(&self, context: Option<CommunityContext>, after: Option<CommunityId>) -> Result<Vec<Community>> {
        self.community_repository
            .list(&context, &after).await
            .map_err(|err| err.into())
    }

    pub async fn list_comments(&self, reply_to: PostId, after: Option<CommentId>) -> Result<Vec<Comment>> {
        self.comment_repository
            .list(&reply_to, &after).await
            .map_err(|err| err.into())
    }

    pub async fn list_feed(&self, feed: Feed, after: Option<PostId>) -> Result<FeedFragment> {
        self.feed_repository
            .list(&feed, &after).await
            .map_err(|err| err.into())

    }
}