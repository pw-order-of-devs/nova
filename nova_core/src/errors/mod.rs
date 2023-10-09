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
    InternalError {
        /// error message
        message: String,
    },
    /// IoError
    IoError {
        /// error message
        message: String,
    },
    /// Resource Not Found
    NotFound,
    /// ParseError
    ParseError {
        /// error message
        message: String,
    },
    /// Unauthorized
    Unauthorized,
    /// UnsupportedRequestType
    UnsupportedRequestType,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest { message } => write!(f, "Bad Request: {message}"),
            Self::EmptyRequest => write!(f, "Empty Request"),
            Self::InternalError { message } => write!(f, "Internal Error: {message}"),
            Self::IoError { message } => write!(f, "IO Error: {message}"),
            Self::NotFound => write!(f, "Not Found"),
            Self::ParseError { message } => {
                write!(f, "Parse Request Error: {message}")
            }
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::UnsupportedRequestType => write!(f, "Unsupported Request Type"),
        }
    }
}

impl std::error::Error for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError {
            message: value.to_string(),
        }
    }
}

impl From<FromUtf8Error> for ServerError {
    fn from(value: FromUtf8Error) -> Self {
        Self::ParseError {
            message: value.to_string(),
        }
    }
}
