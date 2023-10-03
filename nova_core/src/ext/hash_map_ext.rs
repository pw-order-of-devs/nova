use std::collections::HashMap;

use crate::errors::ServerError;

pub(crate) trait HashMapExt {
    fn new(inner: HashMap<String, String>) -> Self where Self: Sized;
    fn get_inner(&self) -> HashMap<String, String>;

    fn contains_key(&mut self, key: &str) -> bool {
        self.get_inner().keys().any(|k| k.to_lowercase() == key.to_lowercase())
    }

    fn get(&self, key: &str) -> Option<String> {
        match self.get_inner().get(key) {
            Some(item) => Some(item.to_string()),
            None => None,
        }
    }

    fn insert(&mut self, _: &str, _: &str) {
        todo!()
    }

    fn insert_if_not_exists(&mut self, _: &str, _: &str) {
        todo!()
    }

    fn from_str(_: &str) -> Result<Self, ServerError> where Self: Sized {
        todo!()
    }

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
