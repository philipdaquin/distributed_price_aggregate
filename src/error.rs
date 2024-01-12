
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;

use serde_json::error::Error as SerdeError;
use std::io;
use binance_spot_connector_rust::hyper::Error as HyperError;

#[derive(Debug, Error)]
pub enum ServerError { 
    #[error("Input / Output operation fails: {0:#?}")]
    IoError(#[source] io::Error), 

    #[error("Input / Output operation fails: {0:#?}")]
    MissingCacheFile(#[source] io::Error),

    #[error("Input / Output operation fails: {0:#?}")]
    SerialisationError(#[source] SerdeError),

    #[error("Input / Output operation fails")]
    HttpClientError,

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
impl From<HyperError> for ServerError { 
    fn from(value: HyperError) -> Self {
        ServerError::HttpClientError
    }
}