use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::NovaError;

/// http request type
#[derive(Debug)]
pub enum RequestType {
    /// HTTP::GET
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
    type Err = NovaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(RequestType::Get),
            "HEAD" => Ok(RequestType::Head),
            "OPTIONS" => Ok(RequestType::Options),
            "POST" => Ok(RequestType::Post),
            "PUT" => Ok(RequestType::Put),
            "PATCH" => Ok(RequestType::Patch),
            "DELETE" => Ok(RequestType::Delete),
            _ => Err(NovaError::UnsupportedRequestType),
        }
    }
}

impl Display for RequestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RequestType::Get => "GET",
            RequestType::Head => "HEAD",
            RequestType::Options => "OPTIONS",
            RequestType::Post => "POST",
            RequestType::Put => "PUT",
            RequestType::Patch => "PATCH",
            RequestType::Delete => "DELETE",
        })
    }
}
