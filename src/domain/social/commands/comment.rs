use crate::domain::social::aggregates::{CommentId, CommentText, PostId};
use crate::domain::account::aggregates::UserId;

pub struct PublishComment {
    pub reply_to: PostId,
    pub text: CommentText,
    pub author: UserId,
}

pub struct PublishCommentResult {
    pub id: CommentId
}

pub struct RemoveComment {
    pub comment: CommentId
}