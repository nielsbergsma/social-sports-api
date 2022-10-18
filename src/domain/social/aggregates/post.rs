use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::social::aggregates::CommunityId;
use crate::domain::social::aggregates::{PostAttachments, PostId, PostText};
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub community: CommunityId,
    pub text: PostText,
    pub attachments: PostAttachments,
    pub author: UserId,
    pub published: DateTime<Utc>,
}
// note: reactions + comments have inversed relationship

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Post {}

impl Post {
    pub fn new(
        id: PostId,
        community: CommunityId,
        text: PostText,
        attachments: PostAttachments,
        author: UserId,
        published: DateTime<Utc>) -> Post {

        Post {
            id,
            community,
            text,
            attachments,
            author,
            published
        }
    }
}