use std::collections::HashMap;

use crate::errors::ServerError;

pub(crate) trait HashMapExt {
    fn new(inner: HashMap<String, String>) -> Self where Self: Sized;
    fn get_inner(&self) -> HashMap<String, String>;

    fn insert(&mut self, k: &str, v: &str) {
        self.get_inner().insert(k.to_string(), v.to_string());
    }

    fn from_str(str: &str) -> Result<Self, ServerError> where Self: Sized;

    fn is_empty(&self) -> bool {
        self.get_inner().is_empty()
    }

    fn print(&self) -> String {
        self.get_inner().iter()
            .map(|item| format!("{}: {}", item.0, item.1))
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}
