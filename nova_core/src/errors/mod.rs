use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Nova Error
#[derive(Clone, Debug)]
pub enum ServerError {
    /// BadRequest
    BadRequest {
        /// error message
        message: String,
    },
    /// EmptyRequest
    EmptyRequest,
    /// InternalError
    InternalError,
    /// IoError
    IoError {
        /// error message
        message: String,
    },
    /// Resource Not Found
    NotFound,
    /// ParseRequestError
    ParseRequestError,
    /// UnsupportedRequestType
    UnsupportedRequestType,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::BadRequest { message } => write!(f, "Bad Request: {message}"),
            ServerError::EmptyRequest => write!(f, "Empty Request"),
            ServerError::InternalError => write!(f, "Internal Error"),
            ServerError::IoError { message } => write!(f, "IO Error: {message}"),
            ServerError::NotFound => write!(f, "Not Found"),
            ServerError::ParseRequestError => write!(f, "Parse Request Error"),
            ServerError::UnsupportedRequestType => write!(f, "Unsupported Request Type"),
        }
    }
}

impl std::error::Error for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        ServerError::IoError { message: value.to_string() }
    }
}

impl From<FromUtf8Error> for ServerError {
    fn from(_: FromUtf8Error) -> Self {
        ServerError::ParseRequestError
    }
}
