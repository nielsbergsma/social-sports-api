use crate::domain::club::aggregates::{ClubId};
use crate::domain::team::aggregates::{TeamId, TeamName};
use crate::domain::account::aggregates::{UserId};

pub struct New {
    pub name: TeamName,
    pub club: ClubId
}

pub struct NewResult {
    pub id: TeamId
}

pub struct AddStaffMember {
    pub team: TeamId,
    pub person: UserId
}

pub struct RemoveStaffMember {
    pub team: TeamId,
    pub staff_member: UserId
}