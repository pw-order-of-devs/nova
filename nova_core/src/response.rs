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
            ServerError::BadRequest { .. } | ServerError::ParseError { .. } => {
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

/// Basic Operations for `HttpResponse`
pub trait HttpResponseData {
    /// Set response protocol
    ///
    /// # Errors
    ///
    /// * wrapper for expected return type
    fn protocol(self, value: Protocol) -> ServerResponse;

    /// Set response status
    ///
    /// # Errors
    ///
    /// * wrapper for expected return type
    fn status(self, value: HttpStatus) -> ServerResponse;

    /// Set response body
    ///
    /// # Errors
    ///
    /// * wrapper for expected return type
    fn body(self, value: &str) -> ServerResponse;

    /// Set response headers
    ///
    /// # Errors
    ///
    /// * wrapper for expected return type
    fn headers(self, value: Headers) -> ServerResponse;

    /// Append response headers
    ///
    /// # Errors
    ///
    /// * wrapper for expected return type
    fn header(self, k: &str, v: &str) -> ServerResponse;
}

impl HttpResponseData for HttpResponse {
    fn protocol(mut self, value: Protocol) -> ServerResponse {
        self.protocol = value;
        Ok(self)
    }

    fn status(mut self, value: HttpStatus) -> ServerResponse {
        self.status = value;
        Ok(self)
    }

    fn body(mut self, value: &str) -> ServerResponse {
        self.body = value.to_string();
        Ok(self)
    }

    fn headers(mut self, value: Headers) -> ServerResponse {
        self.headers = value;
        Ok(self)
    }

    fn header(mut self, k: &str, v: &str) -> ServerResponse {
        self.headers.insert(k, v);
        Ok(self)
    }
}

impl HttpResponseData for ServerResponse {
    fn protocol(self, value: Protocol) -> ServerResponse {
        self?.protocol(value)
    }

    fn status(self, value: HttpStatus) -> ServerResponse {
        self?.status(value)
    }

    fn body(self, value: &str) -> ServerResponse {
        self?.body(value)
    }

    fn headers(self, value: Headers) -> ServerResponse {
        self?.headers(value)
    }

    fn header(self, k: &str, v: &str) -> ServerResponse {
        self?.header(k, v)
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
