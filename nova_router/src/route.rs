use std::fmt::{Debug, Formatter};
use nova_core::request::HttpRequest;

use crate::callable::{CloneableFn, ServerResponse};

/// Nova Route structure
#[derive(Clone)]
pub struct Route {
    r#type: String,
    path: String,
    f: Box<dyn CloneableFn<Output=ServerResponse>>,
}

impl Route {
    /// create new route
    pub fn new<F: CloneableFn<Output=ServerResponse> + 'static>(r#type: &str, path: &str, f: F) -> Self {
        Self {
            r#type: r#type.to_string(),
            path: path.to_string(),
            f: Box::new(f),
        }
    }

    /// check if route matches predicate
    pub fn matches(&self, r#type: &str, path: &str) -> bool {
        self.r#type == r#type && self.path == path
    }

    /// call handler
    pub fn call(&mut self, request: HttpRequest) -> ServerResponse {
        (self.f)(request)
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#type, self.path)
    }
}
