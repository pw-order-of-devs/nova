use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use nova_core::types::request_type::RequestType;

use crate::route::Route;

/// Nova Route wrapper
#[derive(Clone, Debug, Default)]
pub struct Routes {
    inner: HashSet<Route>,
}

impl Routes {
    /// Push into routes
    pub fn push(&mut self, route: Route) {
        self.inner.insert(route);
    }

    /// Find route by type and path
    #[must_use]
    pub fn find(&self, r#type: RequestType, path: &str) -> Option<Route> {
        self.clone()
            .inner
            .into_iter()
            .find(|r| r.matches(r#type, path))
    }

    /// Iterate over routes
    pub fn for_each<F: FnMut(Route)>(&self, f: F) {
        self.clone().inner.into_iter().for_each(f);
    }

    fn print(&self) -> String {
        let mut str = vec![];
        for item in self.clone().inner {
            str.push(format!("{} {}", item.get_type(), item.get_path()));
        }
        format!("[{}]", str.join(", "))
    }
}

impl From<Vec<Route>> for Routes {
    fn from(value: Vec<Route>) -> Self {
        let mut inner = HashSet::new();
        for v in value {
            inner.insert(v);
        }
        Self { inner }
    }
}

impl Display for Routes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}
