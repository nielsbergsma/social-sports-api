use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClubId {
    raw: String,
}

#[derive(Debug)]
pub enum ParseError {
    MalformedInput,
}

impl ClubId {
    pub fn random() -> ClubId {
        ClubId {
            raw: friendly_id::create(),
        }
    }

    pub fn parse(input: &str) -> Result<ClubId, ParseError> {
        if input.len() < 18 || input.len() > 22 {
            return Err(ParseError::MalformedInput);
        }

        friendly_id::decode(input)
            .map(|_| ClubId {
                raw: String::from(input),
            })
            .map_err(|_| ParseError::MalformedInput)
    }
}

impl Display for ClubId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
impl Hash for ClubId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}
