use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::ext::hash_map_ext::HashMapExt;

/// Nova Path
#[derive(Clone, Debug, Default)]
pub struct Path {
    inner: HashMap<String, String>,
}

impl HashMapExt for Path {
    fn new(inner: HashMap<String, String>) -> Self {
        Self { inner }
    }

    fn get_inner(&self) -> HashMap<String, String> {
        self.inner.clone()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}
