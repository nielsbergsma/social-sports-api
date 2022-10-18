use crate::domain::social::aggregates::CommunityId;
use crate::domain::social::aggregates::{PostReaction};
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum PostReactionPolicyViolation {
    InsufficientPermissions,
    OperationBlacklisted,
}

type PostReactionPolicyResult = Result<(), PostReactionPolicyViolation>;

pub struct PostReactionPolicyExecutionContext {
    pub user: UserId,
    pub community: CommunityId,
}

pub trait PostReactionPolicy {
    fn allow_react(
        context: &PostReactionPolicyExecutionContext,
        reaction: PostReaction,
    ) -> PostReactionPolicyResult;
}
