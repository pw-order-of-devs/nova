use std::fmt::{Display, Formatter};

use crate::errors::ServerError;
use crate::types::headers::Headers;
use crate::types::status::HttpStatus;

/// Nova Response definition
#[derive(Clone, Debug)]
pub struct HttpResponse {
    protocol: String,
    status: HttpStatus,
    body: String,
    headers: Headers,
}

impl HttpResponse {
    /// Build Nova Response
    pub fn build(status: HttpStatus, body: &str, headers: Headers, protocol: &str) -> Self {
        HttpResponse {
            protocol: protocol.to_string(),
            status,
            body: body.to_string(),
            headers,
        }
    }

    /// Build Nova Response from NovaError
    pub fn from_error(e: ServerError, protocol: &str) -> Self {
        let (status, body) = match e {
            ServerError::EmptyRequest => (HttpStatus::BadRequest, "Empty request"),
            ServerError::InternalError => (HttpStatus::InternalServerError, "Internal error"),
            ServerError::IoError { .. } => (HttpStatus::InternalServerError, "IO error"),
            ServerError::NotFound { .. } => (HttpStatus::NotFound, "Not Found"),
            ServerError::ParseRequestError => (HttpStatus::BadRequest, "Bad request"),
            ServerError::UnsupportedRequestType => (HttpStatus::MethodNotAllowed, "Unsupported request type"),
        };
        let mut headers = Headers::default();
        headers.insert("Content-length", &body.len().to_string());

        HttpResponse {
            protocol: protocol.to_string(),
            status,
            body: body.to_string(),
            headers,
        }
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = vec![];
        errors.push(write!(f, "{} {}", self.protocol, self.status));
        if !self.headers.is_empty() { errors.push(write!(f, "\r\n{}", self.headers)); }
        if !self.body.is_empty() { errors.push(write!(f, "\r\n\r\n{}", self.body)); }

        if !errors.is_empty() { errors[0] }
        else { Ok(()) }
    }
}
