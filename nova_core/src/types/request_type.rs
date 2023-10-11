use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::ServerError;

/// HTTP `RequestType`
#[derive(Clone, Copy, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum RequestType {
    /// HTTP::GET
    #[default]
    Get,
    /// HTTP::HEAD
    Head,
    /// HTTP::OPTIONS
    Options,
    /// HTTP::POST
    Post,
    /// HTTP::PUT
    Put,
    /// HTTP::PATCH
    Patch,
    /// HTTP::DELETE
    Delete,
}

impl FromStr for RequestType {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            _ => Err(ServerError::UnsupportedRequestType),
        }
    }
}

impl Display for RequestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Head => "HEAD",
                Self::Options => "OPTIONS",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Patch => "PATCH",
                Self::Delete => "DELETE",
            }
        )
    }
}
