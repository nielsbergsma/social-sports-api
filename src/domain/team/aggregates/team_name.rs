use std::fmt::{Display, Formatter};
use chumsky::prelude::{end, filter, Parser, Simple};
use chumsky::text::TextParser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TeamName {
    raw: String,
}

#[derive(Debug)]
pub enum ParseError {
    MalformedInput,
}

impl TeamName {
    fn parser() -> impl Parser<char, TeamName, Error = Simple<char>> {
        // name can consists of alphanumeric (unicode), space, single quotes and dashes
        // length must be between 2 and 100 characters (runes)
        filter(|c: &char| c.is_alphanumeric() || *c == ' ' || *c == '\'' || *c == '-')
            .repeated()
            .at_least(2)
            .at_most(100)
            .padded()
            .collect::<String>()
            .map(|name| TeamName { raw: name })
            .then_ignore(end())
    }

    pub fn parse(input: &str) -> Result<TeamName, ParseError> {
        TeamName::parser()
            .parse(input)
            .map_err(|_| ParseError::MalformedInput)
    }
}

impl Display for TeamName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}