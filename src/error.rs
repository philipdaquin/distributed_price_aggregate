
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;
use serde_json::error::Error as SerdeError;
use tungstenite::Error as WebSocketError;
use std::{io, num::ParseFloatError};
use binance_spot_connector_rust::{hyper::Error as HyperError};

#[derive(Debug, Error)]
pub enum ServerError { 
    #[error("Input / Output operation fails: {0:#?}")]
    IoError(#[source] io::Error), 

    #[error("Missing Cache File: {0:#?}")]
    MissingCacheFile(#[source] io::Error),

    #[error("Input / Output operation fails: {0:#?}")]
    DeserialisationError(#[source] SerdeError),

    #[error("Input / Output operation fails: {0:#?}")]
    SerdeDeserialisationError(#[source] SerdeError),

    #[error("Input / Output operation fails: {0:#?}")]
    WebSocketConnectionError(#[source] WebSocketError),
    
    #[error("Input / Output operation fails: {0:#?}")]
    FailedToParseInt(#[source] ParseFloatError),

    #[error("Input / Output operation fails")]
    HttpClientError,
    #[error("Input / Output operation fails: {0:#?}")]
    DatabaseWriterFailure(#[source] io::Error), 

    #[error("Input / Output operation fails: {0:#?}")]
    ClientWaitingTooLong(#[source] io::Error),

    #[error("Unexpected command type")]
    UnexpectedCommandType
}

impl From<SerdeError> for ServerError { 
    fn from(value: SerdeError) -> Self {
        ServerError::DeserialisationError(value)
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

impl From<ParseFloatError> for ServerError {
    fn from(value: ParseFloatError) -> Self {
        ServerError::FailedToParseInt(value)

    }
}
impl From<WebSocketError> for ServerError {
    fn from(value: WebSocketError) -> Self {
        ServerError::WebSocketConnectionError(value)

    }
}