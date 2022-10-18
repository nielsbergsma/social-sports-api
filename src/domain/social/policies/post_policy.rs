use crate::domain::social::aggregates::CommunityId;
use crate::domain::social::aggregates::Post;
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum PostPolicyViolation {
    InsufficientPermissions,
}

type PostPolicyResult = Result<(), PostPolicyViolation>;

pub struct PostPolicyExecutionContext {
    pub user: UserId,
    pub community: CommunityId,
}

pub trait PostPolicy {
    fn allow_publish(context: &PostPolicyExecutionContext) -> PostPolicyResult;
    fn allow_remove(context: &PostPolicyExecutionContext, post: &Post) -> PostPolicyResult;
}
