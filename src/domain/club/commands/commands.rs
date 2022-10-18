use crate::domain::club::aggregates::{ClubId, ClubName};
use crate::domain::media::aggregates::{ImageId};
use crate::domain::account::aggregates::{UserId};

pub struct New {
    pub name: ClubName
}

pub struct NewResult {
    pub id: ClubId
}

pub struct SetLogo {
    pub club: ClubId,
    pub logo: ImageId
}

pub struct AddStaffMember {
    pub club: ClubId,
    pub person: UserId
}

pub struct RemoveStaffMember {
    pub club: ClubId,
    pub staff_member: UserId
}