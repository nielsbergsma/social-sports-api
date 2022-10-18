use crate::domain::club::aggregates::{ClubId, ClubName};
use crate::domain::media::aggregates::ImageId;

use crate::domain::account::aggregates::UserId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Club {
    pub id: ClubId,
    pub name: ClubName,
    pub logo: Option<ImageId>,
    pub staff: HashSet<UserId>, //TODO: record role(s) of member of staff
}

// note: relation club -> 1:0+ -> teams is inverse (team -> club)
impl Club {
    pub fn new(id: ClubId, name: ClubName) -> Club {
        Club {
            id,
            name,
            logo: Option::None,
            staff: HashSet::new(),
        }
    }

    pub fn set_logo(&mut self, logo: &ImageId) {
        self.logo = Option::Some(logo.clone())
    }

    pub fn add_staff_member(&mut self, person: &UserId) -> bool {
        self.staff.insert(person.clone())
    }

    pub fn remove_staff_member(&mut self, staff_member: &UserId) -> bool {
        self.staff.remove(staff_member)
    }
}

impl PartialEq for Club {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Club {}
