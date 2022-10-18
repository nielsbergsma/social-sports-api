use crate::domain::social::aggregates::{PostReaction};

pub struct ReactToPost {
    pub reaction: PostReaction,
}

pub struct RetractPostReaction {
    pub reaction: PostReaction,
}