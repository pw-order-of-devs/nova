use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::errors::ServerError;
use crate::ext::hash_map_ext::HashMapExt;
use crate::validators::has_error::ValidateHasError;

/// Nova `Query`
#[derive(Clone, Debug, Default)]
pub struct Query {
    inner: HashMap<String, String>,
}

impl HashMapExt for Query {
    fn new(inner: HashMap<String, String>) -> Self {
        Self { inner }
    }

    fn get_inner(&self) -> HashMap<String, String> {
        self.inner.clone()
    }

    fn from_str(str: &str) -> Result<Self, ServerError> {
        let inner = str
            .split('&')
            .map(|i| {
                let item = i.split('=').collect::<Vec<&str>>();
                if item.len() < 2 {
                    Err(ServerError::BadRequest {
                        message: "unable to parse query string".to_string(),
                    })
                } else {
                    Ok((item[0].to_string(), item[1].to_string()))
                }
            })
            .collect::<Vec<_>>()
            .has_error()?
            .into_iter()
            .map(Result::unwrap)
            .collect();
        Ok(Self { inner })
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}
