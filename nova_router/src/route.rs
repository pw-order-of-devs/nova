use std::fmt::{Debug, Formatter};

use nova_core::request::HttpRequest;
use nova_core::types::request_type::RequestType;

use crate::callable::{CloneableFn, ServerResponse};

/// Nova Route structure
#[derive(Clone)]
pub struct Route {
    r#type: RequestType,
    path: String,
    f: Box<dyn CloneableFn<Output=ServerResponse>>,
}

impl Route {
    /// create new route
    pub fn new<F: CloneableFn<Output=ServerResponse> + 'static>(r#type: RequestType, path: &str, f: F) -> Self {
        Self { r#type, path: path.to_string(), f: Box::new(f), }
    }

    /// extract path string
    pub fn get_path(self) -> String {
        self.path
    }

    /// check if route matches predicate
    pub fn matches(&self, r#type: RequestType, path: &str) -> bool {
        if self.r#type != r#type { return false }
        let self_segments = self.path.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let segments = path.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        if self_segments.len() != segments.len() { return false }
        self_segments.into_iter()
            .zip(segments.into_iter())
            .all(|(s, t)| s == t || (s.starts_with('{') && s.ends_with('}')))
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
