//! Errors for database-related issues
use mongodb::error::Error;
use thiserror::Error;

/// Represents any of the errors that can occur for database-related functions
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("The player already exists")]
    PlayerAlreadyExists,
    #[error("The player does not exist")]
    PlayerDoesNotExist,
    #[error("An internal Mongo error has occurred. See logs.")]
    InternalDBError(#[from] Error),
}
