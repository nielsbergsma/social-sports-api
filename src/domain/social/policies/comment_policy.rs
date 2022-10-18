use crate::domain::social::aggregates::comment::Comment;
use crate::domain::social::aggregates::CommunityId;
use crate::domain::social::aggregates::PostId;
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum CommentPolicyViolation {
    InsufficientPermissions,
}

type CommentPolicyResult = Result<(), CommentPolicyViolation>;

pub struct CommentPolicyExecutionContext {
    pub user: UserId,
    pub community: CommunityId,
    pub post: PostId,
}

pub trait CommentPolicy {
    fn allow_publish(
        context: &CommentPolicyExecutionContext
    ) -> CommentPolicyResult;
    
    fn allow_remove(
        context: &CommentPolicyExecutionContext,
        comment: &Comment,
    ) -> CommentPolicyResult;
}
