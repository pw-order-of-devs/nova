use std::fmt::{Display, Formatter};

/// nova error
#[derive(Debug, Clone)]
pub enum NovaError {
    /// EmptyRequest
    EmptyRequest,
    /// InternalError
    InternalError,
    /// IoError
    IoError {
        /// error message
        message: String,
    },
    /// ParseRequestError
    ParseRequestError,
    /// UnsupportedRequestType
    UnsupportedRequestType,
}

impl Display for NovaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NovaError::EmptyRequest => write!(f, "Empty Request"),
            NovaError::InternalError => write!(f, "Internal Error"),
            NovaError::IoError { message } => write!(f, "IO Error: {message}"),
            NovaError::ParseRequestError => write!(f, "Parse Request Error"),
            NovaError::UnsupportedRequestType => write!(f, "Unsupported Request Type"),
        }
    }
}

impl std::error::Error for NovaError {}

impl From<std::io::Error> for NovaError {
    fn from(value: std::io::Error) -> Self {
        NovaError::IoError { message: value.to_string() }
    }
}
