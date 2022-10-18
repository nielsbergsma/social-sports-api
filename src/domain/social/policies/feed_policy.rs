use crate::domain::social::aggregates::feed::Feed;
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum FeedPolicyViolation {
    InsufficientPermissions,
}

type FeedPolicyResult = Result<(), FeedPolicyViolation>;

pub struct FeedPolicyExecutionContext {
    pub user: UserId,
    pub feed: Feed,
}

pub trait FeedPolicy {
    fn allow_fetch(context: &FeedPolicyExecutionContext) -> FeedPolicyResult;
}
