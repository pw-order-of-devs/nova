use std::fmt::{Display, Formatter};

/// Nova `HTTPStatus`
#[derive(Clone, Copy, Debug, Default)]
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
    #[default]
    OK,
    /// Http Status 201: Created
    Created,
    /// Http Status 202: Accepted
    Accepted,
    /// Http Status 203: Non-Authoritative Information
    NonAuthoritativeInformation,
    /// Http Status 204: No Content
    NoContent,
    /// Http Status 205: Reset Content
    ResetContent,
    /// Http Status 206: Partial Content
    PartialContent,
    /// Http Status 207: Multi-Status
    MultiStatus,
    /// Http Status 300: Multiple Choices
    MultipleChoices,
    /// Http Status 301: Moved Permanently
    MovedPermanently,
    /// Http Status 302: Found
    Found,
    /// Http Status 303: See Other
    SeeOther,
    /// Http Status 304: Not Modified
    NotModified,
    /// Http Status 305: Use Proxy
    UseProxy,
    /// Http Status 306: (Unused)
    Unused,
    /// Http Status 307: Temporary Redirect
    TemporaryRedirect,
    /// Http Status 308: Permanent Redirect
    PermanentRedirect,
    /// Http Status 400: Bad Request
    BadRequest,
    /// Http Status 401: Unauthorized
    Unauthorized,
    /// Http Status 402: Payment Required
    PaymentRequired,
    /// Http Status 403: Forbidden
    Forbidden,
    /// Http Status 404: Not Found
    NotFound,
    /// Http Status 405: Method Not Allowed
    MethodNotAllowed,
    /// Http Status 406: Not Acceptable
    NotAcceptable,
    /// Http Status 407: Proxy Authentication Required
    ProxyAuthenticationRequired,
    /// Http Status 408: Request Timeout
    RequestTimeout,
    /// Http Status 409: Conflict
    Conflict,
    /// Http Status 410: Gone
    Gone,
    /// Http Status 411: Length Required
    LengthRequired,
    /// Http Status 412: Precondition Failed
    PreconditionFailed,
    /// Http Status 413: Request Entity Too Large
    RequestEntityTooLarge,
    /// Http Status 414: Request-URI Too Long
    RequestURITooLong,
    /// Http Status 415: Unsupported Media Type
    UnsupportedMediaType,
    /// Http Status 416: Requested Range Not Satisfiable
    RequestedRangeNotSatisfiable,
    /// Http Status 417: Expectation Failed
    ExpectationFailed,
    /// Http Status 418: I'm a teapot
    ImATeapot,
    /// Http Status 420: Enhance Your Calm
    EnhanceYourCalm,
    /// Http Status 421: Misdirected Request
    MisdirectedRequest,
    /// Http Status 422: Unprocessable Entity
    UnprocessableEntity,
    /// Http Status 423: Locked
    Locked,
    /// Http Status 424: Failed Dependency
    FailedDependency,
    /// Http Status 425: Too Early
    TooEarly,
    /// Http Status 426: Upgrade Required
    UpgradeRequired,
    /// Http Status 428: Precondition Required
    PreconditionRequired,
    /// Http Status 429: Too Many Requests
    TooManyRequests,
    /// Http Status 431: Request Header Fields Too Large
    RequestHeaderFieldsTooLarge,
    /// Http Status 444: No Response
    NoResponse,
    /// Http Status 450: Blocked by Windows Parental Control
    BlockedByWindowsParentalControls,
    /// Http Status 451: Unavailable For Legal Reasons
    UnavailableForLegalReasons,
    /// Http Status 500: Internal Server Error
    InternalServerError,
    /// Http Status 501: Not Implemented
    NotImplemented,
    /// Http Status 502: Bad Gateway
    BadGateway,
    /// Http Status 503: Service Unavailable
    ServiceUnavailable,
    /// Http Status 504: Gateway Timeout
    GatewayTimeout,
    /// Http Status 505: HTTP Version Not Supported
    HTTPVersionNotSupported,
    /// Http Status 506: Variant Also Negotiates
    VariantAlsoNegotiates,
    /// Http Status 507: Insufficient Storage
    InsufficientStorage,
    /// Http Status 508: Loop Detected
    LoopDetected,
    /// Http Status 510: Not Extended
    NotExtended,
    /// Http Status 511: Network Authentication Required
    NetworkAuthenticationRequired,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Continue => write!(f, "100 Continue"),
            Self::SwitchingProtocols => write!(f, "101 Switching Protocols"),
            Self::Processing => write!(f, "102 Processing"),
            Self::EarlyHints => write!(f, "103 Early Hints"),
            Self::OK => write!(f, "200 Ok"),
            Self::Created => write!(f, "201 Created"),
            Self::Accepted => write!(f, "202 Accepted"),
            Self::NonAuthoritativeInformation => {
                write!(f, "203 Non-Authoritative Information")
            }
            Self::NoContent => write!(f, "204 No Content"),
            Self::ResetContent => write!(f, "205 Reset Content"),
            Self::PartialContent => write!(f, "206 Partial Content"),
            Self::MultiStatus => write!(f, "207 Multi-Status"),
            Self::MultipleChoices => write!(f, "300 Multiple Choices"),
            Self::MovedPermanently => write!(f, "301 Moved Permanently"),
            Self::Found => write!(f, "302 Found"),
            Self::SeeOther => write!(f, "303 See Other"),
            Self::NotModified => write!(f, "304 Not Modified"),
            Self::UseProxy => write!(f, "305 Use Proxy"),
            Self::Unused => write!(f, "306 Unused"),
            Self::TemporaryRedirect => write!(f, "307 Temporary Redirect"),
            Self::PermanentRedirect => write!(f, "308 Permanent Redirect"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::PaymentRequired => write!(f, "402 Payment Required"),
            Self::Forbidden => write!(f, "403 Forbidden"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            Self::NotAcceptable => write!(f, "406 Not Acceptable"),
            Self::ProxyAuthenticationRequired => {
                write!(f, "407 Proxy Authentication Required")
            }
            Self::RequestTimeout => write!(f, "408 Request Timeout"),
            Self::Conflict => write!(f, "409 Conflict"),
            Self::Gone => write!(f, "410 Gone"),
            Self::LengthRequired => write!(f, "411 Length Required"),
            Self::PreconditionFailed => write!(f, "412 Precondition Failed"),
            Self::RequestEntityTooLarge => write!(f, "413 Request Entity Too Large"),
            Self::RequestURITooLong => write!(f, "414 Request-URI Too Long"),
            Self::UnsupportedMediaType => write!(f, "415 Unsupported Media Type"),
            Self::RequestedRangeNotSatisfiable => {
                write!(f, "416 Requested Range Not Satisfiable")
            }
            Self::ExpectationFailed => write!(f, "417 Expectation Failed"),
            Self::ImATeapot => write!(f, "418 I'm a teapot"),
            Self::EnhanceYourCalm => write!(f, "420 Enhance Your Calm"),
            Self::MisdirectedRequest => write!(f, "421 Misdirected Request"),
            Self::UnprocessableEntity => write!(f, "422 Unprocessable Entity"),
            Self::Locked => write!(f, "423 Locked"),
            Self::FailedDependency => write!(f, "424 Failed Dependency"),
            Self::TooEarly => write!(f, "425 Too Early"),
            Self::UpgradeRequired => write!(f, "426 Upgrade Required"),
            Self::PreconditionRequired => write!(f, "428 Precondition Required"),
            Self::TooManyRequests => write!(f, "429 Too Many Requests"),
            Self::RequestHeaderFieldsTooLarge => {
                write!(f, "431 Request Header Fields Too Large")
            }
            Self::NoResponse => write!(f, "444 No Response"),
            Self::BlockedByWindowsParentalControls => {
                write!(f, "450 Blocked By Windows Parental Controls")
            }
            Self::UnavailableForLegalReasons => {
                write!(f, "451 Unavailable For Legal Reasons")
            }
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
            Self::BadGateway => write!(f, "502 Bad Gateway"),
            Self::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            Self::GatewayTimeout => write!(f, "504 Gateway Timeout"),
            Self::HTTPVersionNotSupported => write!(f, "505 HTTP Version Not Supported"),
            Self::VariantAlsoNegotiates => write!(f, "506 Variant Also Negotiates"),
            Self::InsufficientStorage => write!(f, "507 Insufficient Storage"),
            Self::LoopDetected => write!(f, "508 Loop Detected"),
            Self::NotExtended => write!(f, "510 Not Extended"),
            Self::NetworkAuthenticationRequired => {
                write!(f, "511 Network Authentication Required")
            }
        }
    }
}
