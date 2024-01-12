
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;

use serde_json::Error as SerdeError;
use std::io;

#[derive(Debug, Error)]
pub enum ServerError { 
    #[error("Input / Output operation fails: {0:#?}")]
    IoError(#[source] io::Error), 

    #[error("Input / Output operation fails: {0:#?}")]
    MissingCacheFile(#[source] io::Error),

    #[error("Input / Output operation fails: {0:#?}")]
    SerialisationError(#[source] SerdeError),

    #[error("Input / Output operation fails: {0:#?}")]
    FailureToEstablishConnection(#[source] io::Error), 

    #[error("Input / Output operation fails: {0:#?}")]
    DatabaseWriterFailure(#[source] io::Error), 

    #[error("Input / Output operation fails: {0:#?}")]
    ClientWaitingTooLong(#[source] io::Error),

    #[error("Unexpected command type")]
    UnexpectedCommandType
}

impl From<SerdeError> for ServerError { 
    fn from(value: SerdeError) -> Self {
        ServerError::SerialisationError(value)
    }
}

impl From<io::Error> for ServerError { 
    fn from(value: io::Error) -> Self {
        ServerError::IoError(value)
    }
}