use crate::domain::media::aggregates::ImageId;
use crate::domain::social::aggregates::{CommunityContext, CommunityId, CommunityName};
use crate::domain::account::aggregates::UserId;

pub struct New {
    pub name: CommunityName,
    pub context: CommunityContext,
}

pub struct NewResult {
    pub id: CommunityId
}

pub struct SetLogo {
    pub community: CommunityId,
    pub logo: ImageId,
}

pub struct PromoteMemberToEditor {
    pub community: CommunityId,
    pub member: UserId,
}

pub struct DemoteEditor {
    pub community: CommunityId,
    pub editor: UserId,
}

pub struct Join {
    pub community: CommunityId,
    pub person: UserId,
}

pub struct Leave {
    pub community: CommunityId,
    pub member: UserId,
}
