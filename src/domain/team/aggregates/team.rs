use crate::domain::club::aggregates::ClubId;
use crate::domain::team::aggregates::{TeamId, TeamName};
use crate::domain::account::aggregates::UserId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    pub name: TeamName,
    pub club: ClubId,
    pub staff: HashSet<UserId>, //TODO: record role(s) of member of staff
}

impl Team {
    pub fn new(id: TeamId, name: TeamName, club: ClubId) -> Team {
        Team {
            id,
            name,
            club,
            staff: HashSet::new(),
        }
    }

    pub fn add_staff_member(&mut self, person: &UserId) -> bool {
        // policy: should already be a member of staff from the club
        self.staff.insert(person.clone())
    }

    pub fn remove_staff_member(&mut self, staff_member: &UserId) -> bool {
        self.staff.remove(staff_member)
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Team {}
