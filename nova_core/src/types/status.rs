use std::fmt::{Display, Formatter};

/// Nova Http Status
#[derive(Clone, Copy, Debug)]
pub enum HttpStatus {
    /// Http Status 100: Continue
    Continue,
    /// Http Status 101: Switching Protocols
    SwitchingProtocols,
    /// Http Status 102: Processing
    Processing,
    /// Http Status 103: Early Hints
    EarlyHints,
    /// Http Status 200: OK
    Ok,
    /// Http Status 201: Created
    Created,
    /// Http Status 202: Accepted
    Accepted,
    /// Http Status 203: Non-Authoritative Information
    NonAuthoritativeInformation,
    /// Http Status 204: NoContent
    NoContent,
    /// Http Status 206: Partial Content
    PartialContent,
    /// Http Status 207: Multi-Status
    MultiStatus,
    /// Http Status 400: Bad Request
    BadRequest,
    /// Http Status 404: Not Found
    NotFound,
    /// Http Status 405: Method Not Allowed
    MethodNotAllowed,
    /// Http Status 500: Internal Server Error
    InternalError,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Continue => write!(f, "100 Continue"),
            HttpStatus::SwitchingProtocols => write!(f, "101 Switching Protocols"),
            HttpStatus::Processing => write!(f, "102 Processing"),
            HttpStatus::EarlyHints => write!(f, "103 Early Hints"),
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::Created => write!(f, "201 Created"),
            HttpStatus::Accepted => write!(f, "202 Accepted"),
            HttpStatus::NonAuthoritativeInformation => write!(f, "203 Non-Authoritative Information"),
            HttpStatus::NoContent => write!(f, "204 No Content"),
            HttpStatus::PartialContent => write!(f, "206 Partial Content"),
            HttpStatus::MultiStatus => write!(f, "207 Multi-Status"),
            HttpStatus::BadRequest => write!(f, "400 Bad Request"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
            HttpStatus::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            HttpStatus::InternalError => write!(f, "500 Internal Server Error"),
        }
    }
}
