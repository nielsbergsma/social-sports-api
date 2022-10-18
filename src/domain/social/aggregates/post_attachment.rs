use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

use crate::domain::media::aggregates::ImageId;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum PostAttachment {
    Image(ImageId), //TODO: Video(VideoId)
}

// note: attachment is value object (DDD)
// note: its equality is based on type + underlying content
#[derive(Serialize, Deserialize)]
pub struct PostAttachments {
    elements: Vec<PostAttachment>,
}

impl PostAttachments {
    pub const MAX_ELEMENTS: usize = 10;

    pub fn from_vec(elements: Vec<PostAttachment>) -> PostAttachments {
        // this silently drops > MAX_ELEMENTS elements
        // alternatively, could use Result + specialised error to signal behaviour to account
        PostAttachments {
            elements: elements
                .into_iter()
                .unique()
                .take(PostAttachments::MAX_ELEMENTS)
                .collect(),
        }
    }

    pub fn iter(&self) -> Iter<'_, PostAttachment> {
        self.elements.iter()
    }
}
