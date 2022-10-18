use std::fmt::Formatter;

#[derive(Debug)]
pub enum DomainError {
    UnknownCommunity,
    UnknownPost,
    UnknownComment,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::UnknownCommunity => write!(f,"unknown community"),
            DomainError::UnknownPost => write!(f,"unknown post"),
            DomainError::UnknownComment => write!(f,"unknown comment"),
        }
    }
}

impl std::error::Error for DomainError {}
