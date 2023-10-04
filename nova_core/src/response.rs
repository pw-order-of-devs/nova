use std::fmt::{Display, Formatter};

use crate::errors::ServerError;
use crate::ext::hash_map_ext::HashMapExt;
use crate::types::headers::Headers;
use crate::types::protocol::Protocol;
use crate::types::status::HttpStatus;

/// Callable return type
pub type ServerResponse = Result<HttpResponse, ServerError>;

/// Nova Response definition
#[derive(Clone, Debug, Default)]
pub struct HttpResponse {
    protocol: Protocol,
    status: HttpStatus,
    body: String,
    headers: Headers,
}

impl HttpResponse {
    /// Create new `HttpResponse`
    #[must_use]
    pub fn new(status: HttpStatus, body: &str, headers: Headers, protocol: Protocol) -> Self {
        Self {
            protocol,
            status,
            body: body.to_string(),
            headers,
        }
    }

    /// Build Nova Response from `NovaError`
    #[must_use]
    pub fn from_error(e: &ServerError, protocol: Protocol) -> Self {
        let (status, body) = match e {
            ServerError::BadRequest { .. } | ServerError::ParseRequestError { .. } => {
                (HttpStatus::BadRequest, "Bad request")
            }
            ServerError::EmptyRequest => (HttpStatus::BadRequest, "Empty request"),
            ServerError::InternalError => (HttpStatus::InternalServerError, "Internal error"),
            ServerError::IoError { .. } => (HttpStatus::InternalServerError, "IO error"),
            ServerError::NotFound { .. } => (HttpStatus::NotFound, "Not Found"),
            ServerError::UnsupportedRequestType => {
                (HttpStatus::MethodNotAllowed, "Unsupported request type")
            }
        };
        let mut headers = Headers::default();
        headers.insert("Content-length", &body.len().to_string());

        Self {
            protocol,
            status,
            body: body.to_string(),
            headers,
        }
    }

    /// Set response protocol
    #[must_use]
    pub const fn protocol(mut self, value: Protocol) -> Self {
        self.protocol = value;
        self
    }

    /// Set response status
    #[must_use]
    pub const fn status(mut self, value: HttpStatus) -> Self {
        self.status = value;
        self
    }

    /// Set response body
    #[must_use]
    pub fn body(mut self, value: &str) -> Self {
        self.body = value.to_string();
        self
    }

    /// Set response headers
    #[must_use]
    pub fn headers(mut self, value: Headers) -> Self {
        self.headers = value;
        self
    }

    /// Append response headers
    #[must_use]
    pub fn header(mut self, k: &str, v: &str) -> Self {
        self.headers.insert(k, v);
        self
    }

    /// Append default headers if not present
    #[must_use]
    pub fn append_default_headers(mut self) -> Self {
        self.headers
            .insert_if_not_exists("Content-Length", &self.body.len().to_string());
        self.headers
            .insert_if_not_exists("Content-Type", "text/plain; charset=utf-8");
        self.headers
            .insert_if_not_exists("Date", &chrono::Utc::now().to_rfc2822());
        self.clone()
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = vec![];
        errors.push(write!(f, "{} {}", self.protocol, self.status));
        if !self.headers.is_empty() {
            errors.push(write!(f, "\r\n{}", self.headers));
        }
        if !self.body.is_empty() {
            errors.push(write!(f, "\r\n\r\n{}", self.body));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            errors[0]
        }
    }
}
