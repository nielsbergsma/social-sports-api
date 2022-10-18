use std::fmt::{Display, Formatter};
use chumsky::prelude::{filter, Parser, Simple};
use chumsky::text::TextParser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CommentText {
    raw: String,
}

#[derive(Debug)]
pub enum ParseError {
    MalformedInput,
}

impl CommentText {
    fn parser() -> impl Parser<char, CommentText, Error = Simple<char>> {
        // text shoud only contain visible (non-control) characters
        // length between 5 en 2000 characters
        filter(|c: &char| !c.is_ascii_control())
            .repeated()
            .at_least(5)
            .at_most(2000)
            .padded()
            .collect::<String>()
            .map(|name| CommentText { raw: name })
    }

    pub fn parse(input: &str) -> Result<CommentText, ParseError> {
        CommentText::parser()
            .parse(input)
            .map_err(|_| ParseError::MalformedInput)
    }
}

impl Display for CommentText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}