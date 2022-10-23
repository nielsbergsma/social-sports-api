use crate::domain::social::aggregates::{CommunityId, Post};
use crate::domain::account::aggregates::UserId;
use std::slice::Iter;

// note: feed & friends are transient; meaning they should be derived and not persisted
pub enum Feed {
    Memberships(UserId),
    Community(CommunityId),
}

pub struct FeedListing {
    pub post: Post,
    pub comments: u64,
    pub reactions_love: u64,
    pub reactions_funny: u64,
    pub reactions_celebrate: u64,
    pub reactions_support: u64,
    pub reactions_insightful: u64,
}

pub struct FeedFragment {
    listings: Vec<FeedListing>,
}

impl FeedFragment {
    pub const MAX_ELEMENTS: usize = 25;

    pub fn from_vec(listings: Vec<FeedListing>) -> FeedFragment {
        FeedFragment {
            listings: listings
                .into_iter()
                .take(FeedFragment::MAX_ELEMENTS)
                .collect(),
        }
    }

    pub fn iter(&self) -> Iter<'_, FeedListing> {
        self.listings.iter()
    }
}
