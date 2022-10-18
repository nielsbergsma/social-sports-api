use crate::domain::club::aggregates::Club;
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum ClubPolicyViolation {
    InsufficientPermissions,
}

type ClubPolicyResult = Result<(), ClubPolicyViolation>;

pub struct ClubPolicyExecutionContext {
    pub club: Club,
    pub user: UserId,
}

pub trait ClubPolicy {
    fn allow_new(context: &ClubPolicyExecutionContext) -> ClubPolicyResult;
    fn allow_set_logo(context: &ClubPolicyExecutionContext) -> ClubPolicyResult;
    fn allow_add_staff_member(
        context: &ClubPolicyExecutionContext,
        person: &UserId,
    ) -> ClubPolicyResult;
    fn allow_demote_staff_member(
        context: &ClubPolicyExecutionContext,
        person: &UserId,
    ) -> ClubPolicyResult;
}
