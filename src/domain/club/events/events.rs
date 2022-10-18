use serde::{Deserialize, Serialize};
use crate::common::Event;
use crate::domain::club::aggregates::{ClubId, ClubName};
use crate::domain::media::aggregates::ImageId;
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize)]
pub struct ClubAddedV1 {
    pub id:  ClubId,
    pub name: ClubName,
}

impl Event for ClubAddedV1 {
    fn kind(&self) -> &'static str {
        "ClubAddedV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClubLogoSetV1 {
    pub club: ClubId,
    pub logo: ImageId,
}

impl Event for ClubLogoSetV1 {
    fn kind(&self) -> &'static str {
        "ClubLogoSetV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct StaffMemberAddedToClubV1 {
    pub club: ClubId,
    pub person: UserId,
}

impl Event for StaffMemberAddedToClubV1 {
    fn kind(&self) -> &'static str {
        "StaffMemberAddedToClubV1"
    }
}

#[derive(Serialize, Deserialize)]
pub struct StaffMemberRemovedFromClubV1 {
    pub club: ClubId,
    pub staff_member: UserId,
}

impl Event for StaffMemberRemovedFromClubV1 {
    fn kind(&self) -> &'static str {
        "StaffMemberRemovedFromClubV1"
    }
}