use std::fmt::Formatter;
use crate::common::RepositoryError::{StorageError, UnknownError};

#[derive(Debug)]
pub enum RepositoryError {
    StorageError,
    UnknownError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError => write!(f,"storage error"),
            UnknownError(details) => write!(f,"unknown error: {}", details),
        }
    }
}

impl std::error::Error for RepositoryError {}

pub type RepositoryResult<T> = Result<T, RepositoryError>;


