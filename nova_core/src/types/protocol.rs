use std::fmt::{Display, Formatter};

/// Nova Protocol
#[derive(Clone, Copy, Debug, Default)]
pub enum Protocol {
    /// HTTP/1.1 protocol
    #[default] Http1,
    /// HTTP/2 protocol
    Http2,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http1 => write!(f, "HTTP/1.1"),
            Protocol::Http2 => write!(f, "HTTP/2"),
        }
    }
}
