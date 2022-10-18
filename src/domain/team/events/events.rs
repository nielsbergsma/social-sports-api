use serde::{Deserialize, Serialize};
use crate::common::Event;
use crate::domain::club::aggregates::{ClubId};
use crate::domain::team::aggregates::{TeamId, TeamName};
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize)]
pub struct TeamAddedV1 {
    pub id:  TeamId,
    pub name: TeamName,
    pub club: ClubId,
}

impl Event for TeamAddedV1 {
    fn kind(&self) -> &'static str {
        "TeamAddedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct StaffMemberAddedToTeamV1 {
    pub team: TeamId,
    pub person: UserId,
}

impl Event for StaffMemberAddedToTeamV1 {
    fn kind(&self) -> &'static str {
        "StaffMemberAddedToTeamV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct StaffMemberRemovedFromTeamV1 {
    pub team: TeamId,
    pub staff_member: UserId,
}

impl Event for StaffMemberRemovedFromTeamV1 {
    fn kind(&self) -> &'static str {
        "StaffMemberRemovedFromTeamV1"
    }
}