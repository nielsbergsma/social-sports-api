use crate::domain::team::aggregates::Team;
use crate::domain::account::aggregates::UserId;

#[derive(Debug)]
pub enum TeamPolicyViolation {
    InsufficientPermissions,
}

type TeamPolicyResult = Result<(), TeamPolicyViolation>;

pub struct TeamPolicyExecutionContext {
    pub team: Team,
    pub user: UserId,
}

pub trait TeamPolicy {
    fn allow_new(context: &TeamPolicyExecutionContext) -> TeamPolicyResult;
    fn allow_add_staff_member(
        context: &TeamPolicyExecutionContext,
        person: &UserId,
    ) -> TeamPolicyResult;
    fn allow_demote_staff_member(
        context: &TeamPolicyExecutionContext,
        person: &UserId,
    ) -> TeamPolicyResult;
}
