use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use wtransport::error::ConnectionError;

#[derive(Debug)]
pub enum ClientError {
    TimeOut,
    LocallyClosed,
    QuicError,
    StreamOpeningError
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::TimeOut => write!(f, "{:?}", ConnectionError::TimedOut),
            ClientError::LocallyClosed => write!(f, "{:?}", ConnectionError::LocallyClosed),
            ClientError::QuicError => write!(f, "{:?}", ConnectionError::QuicError),
            ClientError::StreamOpeningError => write!(f, "{:?}", "Failed to open stream")
        }
    }
}

impl Error for ClientError {}
