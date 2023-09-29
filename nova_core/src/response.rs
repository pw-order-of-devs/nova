use std::fmt::{Display, Formatter};

use crate::errors::ServerError;
use crate::ext::hash_map_ext::HashMapExt;
use crate::types::headers::Headers;
use crate::types::protocol::Protocol;
use crate::types::status::HttpStatus;

/// Nova Response definition
#[derive(Clone, Debug, Default)]
pub struct HttpResponse {
    protocol: Protocol,
    status: HttpStatus,
    body: String,
    headers: Headers,
}

impl HttpResponse {
    /// Create new HttpResponse
    pub fn new(status: HttpStatus, body: &str, headers: Headers, protocol: Protocol) -> Self {
        HttpResponse { protocol, status, body: body.to_string(), headers, }
    }

    /// Build Nova Response from NovaError
    pub fn from_error(e: ServerError, protocol: Protocol) -> Self {
        let (status, body) = match e {
            ServerError::BadRequest { .. } => (HttpStatus::BadRequest, "Bad request"),
            ServerError::EmptyRequest => (HttpStatus::BadRequest, "Empty request"),
            ServerError::InternalError => (HttpStatus::InternalServerError, "Internal error"),
            ServerError::IoError { .. } => (HttpStatus::InternalServerError, "IO error"),
            ServerError::NotFound { .. } => (HttpStatus::NotFound, "Not Found"),
            ServerError::ParseRequestError => (HttpStatus::BadRequest, "Bad request"),
            ServerError::UnsupportedRequestType => (HttpStatus::MethodNotAllowed, "Unsupported request type"),
        };
        let mut headers = Headers::default();
        headers.insert("Content-length", &body.len().to_string());

        HttpResponse { protocol, status, body: body.to_string(), headers, }
    }

    /// set response protocol
    pub fn protocol(mut self, value: Protocol) -> Self {
        self.protocol = value;
        self
    }

    /// set response status
    pub fn status(mut self, value: HttpStatus) -> Self {
        self.status = value;
        self
    }

    /// set response body
    pub fn body(mut self, value: &str) -> Self {
        self.body = value.to_string();
        self
    }

    /// set response headers
    pub fn headers(mut self, value: Headers) -> Self {
        self.headers = value;
        self
    }

    /// append response headers
    pub fn header(mut self, k: &str, v: &str) -> Self {
        self.headers.insert(k, v);
        self
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
