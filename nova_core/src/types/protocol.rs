use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::ServerError;

/// Nova `Protocol`
#[derive(Clone, Copy, Debug, Default)]
pub enum Protocol {
    /// HTTP/1.1 protocol
    #[default]
    Http1,
    /// HTTP/2 protocol
    Http2,
}

impl FromStr for Protocol {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Self::Http1),
            "HTTP/2" => Ok(Self::Http2),
            _ => Err(ServerError::UnsupportedRequestType),
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http1 => write!(f, "HTTP/1.1"),
            Self::Http2 => write!(f, "HTTP/2"),
        }
    }
}
