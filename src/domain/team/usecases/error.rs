use std::fmt::Formatter;

#[derive(Debug)]
pub enum DomainError {
    UnknownTeam,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::UnknownTeam => write!(f, "unknown team"),
        }
    }
}

impl std::error::Error for DomainError {}
