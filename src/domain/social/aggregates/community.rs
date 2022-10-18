use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::domain::club::aggregates::ClubId;
use crate::domain::media::aggregates::ImageId;
use crate::domain::social::aggregates::{CommunityId, CommunityName};
use crate::domain::team::aggregates::TeamId;
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize)]
pub struct Community {
    pub id: CommunityId,
    pub name: CommunityName,
    pub context: CommunityContext,
    pub founded: DateTime<Utc>,
    pub logo: Option<ImageId>,
    pub editors: HashSet<UserId>,
    pub members: HashSet<UserId>, // convenient, but limited scalable (inverse if too slow)
}

// note: "ownership" of a community is derived from its context (e.g. club owner = community owner)
#[derive(Serialize, Deserialize, Clone)]
pub enum CommunityContext {
    Club(ClubId),
    Team(TeamId),
}

impl Community {
    pub fn new(
        id: CommunityId,
        name: CommunityName,
        context: CommunityContext,
        founded: DateTime<Utc>,
    ) -> Community {
        Community {
            id,
            name,
            context,
            founded,
            logo: Option::None,
            editors: HashSet::new(),
            members: HashSet::new(),
        }
    }

    pub fn set_logo(&mut self, logo: &ImageId) {
        self.logo = Option::Some(logo.clone())
    }

    pub fn promote_member_to_editor(&mut self, member: &UserId) -> bool {
        // rule: new editor must be member of the community
        let is_member = self.members.contains(member);
        if is_member {
            self.editors.insert(member.clone());
        }

        is_member
    }

    pub fn demote_editor(&mut self, editor: &UserId) -> bool {
        self.editors.remove(editor)
    }

    pub fn join(&mut self, member: &UserId) -> bool {
        self.members.insert(member.clone())
    }

    pub fn leave(&mut self, member: &UserId) -> bool {
        if self.editors.contains(member) {
            return false
        }
        self.members.remove(member)
    }
}

impl PartialEq for Community {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
