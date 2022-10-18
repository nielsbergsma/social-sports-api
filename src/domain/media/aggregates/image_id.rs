use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ImageId {
    raw: String,
}

#[derive(Debug)]
pub enum ParseError {
    MalformedInput,
}

impl ImageId {
    pub fn random() -> ImageId {
        ImageId {
            raw: friendly_id::create(),
        }
    }

    pub fn parse(input: &str) -> Result<ImageId, ParseError> {
        friendly_id::decode(input)
            .map(|_| ImageId {
                raw: String::from(input),
            })
            .map_err(|_| ParseError::MalformedInput)
    }
}

impl Display for ImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}