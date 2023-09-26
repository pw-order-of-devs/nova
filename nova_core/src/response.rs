use std::fmt::{Display, Formatter};

use crate::errors::NovaError;
use crate::types::headers::Headers;

/// Nova Response definition
#[derive(Debug)]
pub struct NovaResponse {
    protocol: String,
    status_code: u16,
    status_message: String,
    body: String,
    headers: Headers,
}

impl NovaResponse {
    /// Build NovaResponse from NovaError
    pub fn from_error(e: NovaError, protocol: &str) -> Self {
        let (status_code, status_message, body) = match e {
            NovaError::EmptyRequest => (400, "Bad Request", "Empty request"),
            NovaError::InternalError => (500, "Internal Server Error", "Internal error"),
            NovaError::IoError { .. } => (500, "Internal Server Error", "IO error"),
            NovaError::ParseRequestError => (400, "Bad Request", "Bad request"),
            NovaError::UnsupportedRequestType => (405, "Method Not Allowed", "Unsupported request type"),
        };
        let mut headers = Headers::default();
        headers.insert("Content-length", &body.len().to_string());

        NovaResponse {
            protocol: protocol.to_string(),
            status_code,
            status_message: status_message.to_string(),
            body: body.to_string(),
            headers,
        }
    }
}

impl Display for NovaResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = vec![];
        errors.push(write!(f, "{} {} {}", self.protocol, self.status_code, self.status_message));
        if !self.headers.is_empty() { errors.push(write!(f, "\r\n{}", self.headers)); }
        if !self.body.is_empty() { errors.push(write!(f, "\r\n\r\n{}", self.body)); }

        if !errors.is_empty() { errors[0] }
        else { Ok(()) }
    }
}
