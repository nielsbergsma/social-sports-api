use std::fmt::{Display, Formatter};
use chumsky::prelude::{filter, Parser, Simple};

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UserId {
    raw: String,
}

#[derive(Debug)]
pub enum ParseError {
    MalformedInput,
}

impl UserId {
    fn parser() -> impl Parser<char, UserId, Error = Simple<char>> {
        // account id (firebase auth) consists of 20-24 alphanumeric ascii characters
        filter(|c: &char| c.is_ascii_alphanumeric())
            .repeated()
            .at_least(20)
            .at_most(24)
            .collect::<String>()
            .map(|name| UserId { raw: name })
    }

    pub fn parse(input: &str) -> Result<UserId, ParseError> {
        UserId::parser()
            .parse(input)
            .map_err(|_| ParseError::MalformedInput)
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}
