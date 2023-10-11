use std::collections::HashMap;

use crate::errors::ServerError;

/// `HashMap` extensions
pub trait HashMapExt {
    /// Create new `HashMap` instance
    fn new(inner: HashMap<String, String>) -> Self
    where
        Self: Sized;

    /// Extract inner value
    fn get_inner(&self) -> HashMap<String, String>;

    /// Check if map contains given key
    fn contains_key(&mut self, key: &str) -> bool {
        self.get_inner()
            .keys()
            .any(|k| k.to_lowercase() == key.to_lowercase())
    }

    /// Extract value by key
    fn get(&self, key: &str) -> Option<String> {
        self.get_inner()
            .iter()
            .find(|(k, _)| key.to_lowercase() == k.to_lowercase())
            .map(|item| item.1.to_string())
    }

    /// Insert value into map
    fn insert(&mut self, _: &str, _: &str) {
        todo!()
    }

    /// Insert value into map if it does not exist
    fn insert_if_not_exists(&mut self, _: &str, _: &str) {
        todo!()
    }

    /// Parse map from string
    ///
    /// # Errors
    ///
    /// Returns a `ServerError` in case of failed parsing of string
    fn from_str(_: &str) -> Result<Self, ServerError>
    where
        Self: Sized,
    {
        todo!()
    }

    /// Check if map is empty
    fn is_empty(&self) -> bool {
        self.get_inner().is_empty()
    }

    /// Print map to string
    fn print(&self) -> String {
        self.get_inner()
            .iter()
            .map(|item| format!("{}: {}", item.0, item.1))
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}
