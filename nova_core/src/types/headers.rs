use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::errors::ServerError;
use crate::ext::hash_map_ext::HashMapExt;
use crate::validators::has_error::ValidateHasError;

/// Nova Headers
#[derive(Clone, Debug, Default)]
pub struct Headers {
    inner: HashMap<String, String>,
}

impl HashMapExt for Headers {
    fn new(inner: HashMap<String, String>) -> Self {
        Self { inner }
    }

    fn get_inner(&self) -> HashMap<String, String> {
        self.inner.clone()
    }

    fn insert(&mut self, k: &str, v: &str) {
        self.inner.insert(k.to_string(), v.to_string());
    }

    fn insert_if_not_exists(&mut self, k: &str, v: &str) {
        if !self.contains_key(k) {
            self.inner.insert(k.to_string(), v.to_string());
        }
    }

    fn from_str(str: &str) -> Result<Self, ServerError> {
        let inner = str
            .split("\r\n")
            .filter(|item| item.contains(": "))
            .map(|item| {
                let item = item.split(": ").collect::<Vec<&str>>();
                if item.len() < 2 {
                    Err(ServerError::BadRequest {
                        message: "unable to parse headers".to_string(),
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
        Ok(Self::new(inner))
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}
