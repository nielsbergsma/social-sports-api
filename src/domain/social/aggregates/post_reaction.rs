use serde::{Deserialize, Serialize};

use crate::domain::social::aggregates::PostId;
use crate::domain::account::aggregates::UserId;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum PostReaction {
    Love(UserId, PostId),
    Funny(UserId, PostId),
    Celebrate(UserId, PostId),
    Support(UserId, PostId),
    Insightful(UserId, PostId),
}

impl PostReaction {
    pub fn values(&self) -> (&str, &UserId, &PostId){
        match self {
            PostReaction::Love(user, post) => ("love", user, post),
            PostReaction::Funny(user, post) => ("funny", user, post),
            PostReaction::Celebrate(user, post) => ("celebrate", user, post),
            PostReaction::Support(user, post) => ("support", user, post),
            PostReaction::Insightful(user, post) => ("insightful", user, post),
        }
    }
}