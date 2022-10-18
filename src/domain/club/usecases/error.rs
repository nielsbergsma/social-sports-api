use std::fmt::Formatter;

#[derive(Debug)]
pub enum DomainError {
    UnknownClub,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::UnknownClub => write!(f, "unknown club"),
        }
    }
}

impl std::error::Error for DomainError {}
