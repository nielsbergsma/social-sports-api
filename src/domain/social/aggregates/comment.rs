use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::social::aggregates::{CommentId, CommentText, PostId};
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: CommentId,
    pub reply_to: PostId,
    pub text: CommentText,
    pub author: UserId,
    pub published: DateTime<Utc>,
}

impl Comment {
    pub fn new(id: CommentId, reply_to: PostId, text: CommentText, author: UserId, published: DateTime<Utc>) -> Comment {
        Comment {
            id,
            reply_to,
            text,
            author,
            published,
        }
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Comment {}
