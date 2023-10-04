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
#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    pub(crate) r#type: RequestType,
    pub(crate) target: String,
    pub(crate) protocol: Protocol,
    pub(crate) query: Query,
    pub(crate) path: Path,
    pub(crate) body: String,
    pub(crate) headers: Headers,
}

impl HttpRequest {
    /// Extract body string from request
    #[must_use]
    pub fn body_string(&self) -> String {
        self.body.clone()
    }

    /// Get path entry by key
    ///
    /// # Errors
    ///
    /// * `ServerError::BadRequest` - requested key is missing in path
    pub fn path(&self, key: &str) -> Result<String, ServerError> {
        self.path.get_inner().get(key).map_or_else(
            || {
                Err(ServerError::BadRequest {
                    message: format!("path item \"{key}\" is missing"),
                })
            },
            |item| Ok(item.clone()),
        )
    }

    /// Get query entry by key
    ///
    /// # Errors
    ///
    /// * `ServerError::BadRequest` - requested key is missing in query
    pub fn query(&self, key: &str) -> Result<String, ServerError> {
        self.query.get_inner().get(key).map_or_else(
            || {
                Err(ServerError::BadRequest {
                    message: format!("query item \"{key}\" is missing"),
                })
            },
            |item| Ok(item.clone()),
        )
    }

    /// Get headers map
    #[must_use]
    pub fn headers(&self) -> Headers {
        self.clone().headers
    }

    /// Get header by key
    #[must_use]
    pub fn header(&self, key: &str) -> Option<String> {
        self.headers.get(key)
    }
}

impl FromStr for HttpRequest {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\r\n").collect::<Vec<&str>>();
        let request = parts[0].split(' ').collect::<Vec<&str>>();
        if request.len() < 3 {
            return Err(ServerError::ParseError {
                message: "request is malformed".to_string(),
            });
        }

        let r#type = RequestType::from_str(request[0])?;
        let target = request[1].to_string();
        let protocol = Protocol::from_str(request[2])?;

        let path = Path::default();
        let query_pos = target.chars().position(|i| i == '?');
        let (target, query) = if let Some(pos) = query_pos {
            (
                target[..pos].to_string(),
                Query::from_str(&target[pos + 1..])?,
            )
        } else {
            (target, Query::default())
        };

        let headers_pos = parts
            .iter()
            .position(|item| item.is_empty())
            .unwrap_or(parts.len() - 1);
        let headers = Headers::from_str(&parts[1..headers_pos].to_vec().join("\r\n"))?;
        let body = parts[headers_pos + 1..parts.len()]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\r\n")
            .chars()
            .filter(|&i| i != '\0')
            .collect();

        Ok(Self {
            r#type,
            target,
            protocol,
            query,
            path,
            body,
            headers,
        })
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut errors = vec![];
        errors.push(write!(
            f,
            "{} {} {}",
            self.r#type, self.target, self.protocol
        ));
        if !self.headers.is_empty() {
            errors.push(write!(f, "\r\nHeaders: \r\n{}", self.headers));
        }
        if !self.body.is_empty() {
            errors.push(write!(f, "\r\nBody: \r\n{}", self.body));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            errors[0]
        }
    }
}
