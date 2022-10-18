use crate::domain::social::aggregates::{CommunityId, PostAttachments, PostId, PostText};
use crate::domain::account::aggregates::UserId;

pub struct PublishPost {
    pub community: CommunityId,
    pub text: PostText,
    pub attachments: PostAttachments,
    pub author: UserId,
}

pub struct PublishPostResult {
    pub id: PostId
}

pub struct RemovePost {
    pub post: PostId
}