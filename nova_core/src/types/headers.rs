use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Nova Headers
#[derive(Debug, Default)]
pub struct Headers {
    inner: HashMap<String, String>,
}

impl Headers {
    /// insert item to Nova Headers instance
    pub fn insert(&mut self, k: &str, v: &str) {
        self.inner.insert(k.to_string(), v.to_string());
    }

    /// build Nova Headers from vec
    pub fn from_vec(vec: &[&str]) -> Self {
        let inner = vec.iter()
            .filter(|item| item.contains(": "))
            .map(|item| {
                let item = item.split(": ").collect::<Vec<&str>>();
                (item[0].to_string(), item[1].to_string())
            })
            .collect();
        Self { inner }
    }

    /// check if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = self.inner.iter()
            .map(|item| format!("{}: {}", item.0, item.1))
            .collect::<Vec<String>>()
            .join("\r\n");
        write!(f, "{string}")
    }
}
