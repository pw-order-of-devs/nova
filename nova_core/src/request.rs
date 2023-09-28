use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::ServerError;
use crate::ext::hash_map_ext::HashMapExt;
use crate::types::headers::Headers;
use crate::types::path::Path;
use crate::types::protocol::Protocol;
use crate::types::query::Query;
use crate::types::request_type::RequestType;

/// Nova Request definition
#[derive(Clone, Debug)]
pub struct HttpRequest {
    r#type: RequestType,
    target: String,
    protocol: Protocol,
    query: Query,
    path: Path,
    body: String,
    headers: Headers,
}

impl HttpRequest {
    /// extract route details
    pub fn get_route_path(&self) -> (RequestType, String) {
        (self.clone().r#type, self.clone().target)
    }

    /// update path map from route
    pub fn update_path(mut self, route: &str) -> Self {
        let self_segments = self.target.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let segments = route.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let inner = self_segments.into_iter()
            .zip(segments.into_iter())
            .filter(|(_, t)| (t.starts_with('{') && t.ends_with('}')))
            .map(|(s, t)| (t[1 .. t.len() - 1].to_string(), s.to_string()))
            .collect();
        self.path = Path::new(inner);
        self
    }

    /// get path by key
    pub fn path(&self, key: &str) -> Result<String, ServerError> {
        match self.path.get_inner().get(key) {
            Some(item) => Ok(item.clone()),
            None => Err(ServerError::BadRequest { message: format!("path item \"{key}\" is missing") }),
        }
    }

    /// get query by key
    pub fn query(&self, key: &str) -> Result<String, ServerError> {
        match self.query.get_inner().get(key) {
            Some(item) => Ok(item.clone()),
            None => Err(ServerError::BadRequest { message: format!("query item \"{key}\" is missing") }),
        }
    }
}

impl FromStr for HttpRequest {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\r\n").collect::<Vec<&str>>();
        let request = parts[0].split(' ').collect::<Vec<&str>>();
        if request.len() < 3 { return Err(ServerError::ParseRequestError) }

        let protocol = Protocol::from_str(request[2])?;
        let r#type = RequestType::from_str(request[0])?;
        let target = request[1].to_string();
        let query_pos = target.chars().position(|i| i == '?');
        let (target, query) =
            if let Some(pos) = query_pos { (target[.. pos].to_string(), Query::from_str(&target[pos + 1..])?) }
            else { (target, Query::default()) };
        let headers = Headers::from_str(parts[0])?;

        Ok(HttpRequest {
            r#type, target, protocol, query,
            path: Default::default(),
            body: "".to_string(),
            headers,
        })
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = vec![];
        errors.push(write!(f, "{} {} {}", self.r#type, self.target, self.protocol));
        if !self.headers.is_empty() { errors.push(write!(f, "\r\nHeaders: \r\n{}", self.headers)); }
        if !self.body.is_empty() { errors.push(write!(f, "\r\nBody: \r\n{}", self.body)); }

        if !errors.is_empty() { errors[0] }
        else { Ok(()) }
    }
}
