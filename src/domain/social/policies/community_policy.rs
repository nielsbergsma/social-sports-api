use crate::domain::social::aggregates::Community;
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum CommunityPolicyViolation {
    InsufficientPermissions,
}

type CommunityPolicyResult = Result<(), CommunityPolicyViolation>;

pub struct CommunityPolicyExecutionContext {
    pub user: UserId,
    pub community: Community,
}

pub trait CommunityPolicy {
    fn allow_new(context: &CommunityPolicyExecutionContext) -> CommunityPolicyResult;
    fn allow_promote_member_to_editor(
        context: &CommunityPolicyExecutionContext,
        member: &UserId,
    ) -> CommunityPolicyResult;
    fn allow_demote_editors(
        context: &CommunityPolicyExecutionContext,
        moderator: &UserId,
    ) -> CommunityPolicyResult;
}
