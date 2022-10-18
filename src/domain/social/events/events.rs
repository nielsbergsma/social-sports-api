use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::common::Event;
use crate::domain::media::aggregates::ImageId;
use crate::domain::social::aggregates::{CommentId, CommentText, CommunityContext, CommunityId, CommunityName, PostAttachments, PostId, PostReaction, PostText};
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize)]
pub struct CommunityAddedV1 {
    pub id:  CommunityId,
    pub name: CommunityName,
    pub context: CommunityContext,
    pub founded: DateTime<Utc>,
}

impl Event for CommunityAddedV1 {
    fn kind(&self) -> &'static str {
        "CommunityAddedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommunityLogoSetV1 {
    pub community:  CommunityId,
    pub logo: ImageId,
}

impl Event for CommunityLogoSetV1 {
    fn kind(&self) -> &'static str {
        "CommunityLogoSetV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemberPromotedToEditorV1 {
    pub community:  CommunityId,
    pub member: UserId,
}

impl Event for MemberPromotedToEditorV1 {
    fn kind(&self) -> &'static str {
        "MemberPromotedToEditorV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct EditorDemotedV1 {
    pub community:  CommunityId,
    pub editor: UserId,
}

impl Event for EditorDemotedV1 {
    fn kind(&self) -> &'static str {
        "EditorDemotedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct JoinedV1 {
    pub community:  CommunityId,
    pub person: UserId,
}

impl Event for JoinedV1 {
    fn kind(&self) -> &'static str {
        "JoinedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct LeftV1 {
    pub community:  CommunityId,
    pub member: UserId,
}

impl Event for LeftV1 {
    fn kind(&self) -> &'static str {
        "LeftV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostPublishedV1 {
    pub id: PostId,
    pub community: CommunityId,
    pub text: PostText,
    pub attachments: PostAttachments,
    pub author: UserId,
    pub published: DateTime<Utc>,
}

impl Event for PostPublishedV1 {
    fn kind(&self) -> &'static str {
        "PostPublishedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostRemovedV1 {
    pub id: PostId,
}

impl Event for PostRemovedV1 {
    fn kind(&self) -> &'static str {
        "PostRemovedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentPublishedV1 {
    pub id: CommentId,
    pub reply_to: PostId,
    pub text: CommentText,
    pub author: UserId,
    pub published: DateTime<Utc>,
}

impl Event for CommentPublishedV1 {
    fn kind(&self) -> &'static str {
        "CommentPublishedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentRemovedV1 {
    pub id: CommentId,
}

impl Event for CommentRemovedV1 {
    fn kind(&self) -> &'static str {
        "CommentRemovedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReactedToPostV1 {
    pub reaction: PostReaction,
}

impl Event for ReactedToPostV1 {
    fn kind(&self) -> &'static str {
        "ReactedToPostV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostReactionRetractedV1 {
    pub reaction: PostReaction,
}

impl Event for PostReactionRetractedV1 {
    fn kind(&self) -> &'static str {
        "PostReactionRetractedV1"
    }
}