use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::NovaError;
use crate::types::headers::Headers;
use crate::types::request_type::RequestType;

/// Nova Request definition
#[derive(Debug)]
pub struct NovaRequest {
    r#type: RequestType,
    target: String,
    protocol: String,
    _query: HashMap<String, String>,
    _path: HashMap<String, String>,
    body: String,
    headers: Headers,
}

impl FromStr for NovaRequest {
    type Err = NovaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\r\n").collect::<Vec<&str>>();
        let request = parts[0].split(' ').collect::<Vec<&str>>();
        let headers = Headers::from_vec(&request);
        let request = NovaRequest {
            r#type: RequestType::from_str(request[0])?,
            target: request[1].to_string(),
            protocol: request[2].to_string(),
            _query: Default::default(),
            _path: Default::default(),
            body: "".to_string(),
            headers,
        };
        Ok(request)
    }
}

impl Display for NovaRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = vec![];
        errors.push(write!(f, "{} {} {}", self.r#type, self.target, self.protocol));
        if !self.headers.is_empty() { errors.push(write!(f, "\r\nHeaders: \r\n{}", self.headers)); }
        if !self.body.is_empty() { errors.push(write!(f, "\r\nBody: \r\n{}", self.body)); }

        if !errors.is_empty() { errors[0] }
        else { Ok(()) }
    }
}
