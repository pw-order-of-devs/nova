use std::fmt::{Debug, Formatter};

use nova_core::request::HttpRequest;
use nova_core::types::request_type::RequestType;

use crate::callable::{CloneableFn, ServerResponse};

/// Nova Route structure
#[derive(Clone)]
pub struct Route {
    r#type: RequestType,
    path: String,
    f: Option<Box<dyn CloneableFn<Output=ServerResponse>>>,
    routes: Vec<Route>,
}

#[allow(clippy::self_named_constructors)]
impl Route {
    /// create new service route
    pub fn service(path: &str, routes: Vec<Route>) -> Self {
        Self { r#type: RequestType::Get, path: path.to_string(), f: None, routes, }
    }

    /// create new route
    pub fn route<F: CloneableFn<Output=ServerResponse> + 'static>(r#type: RequestType, path: &str, f: F) -> Self {
        Self { r#type, path: path.to_string(), f: Some(Box::new(f)), routes: vec![], }
    }

    /// get request type
    pub fn get_type(self) -> RequestType {
        self.r#type
    }

    /// get path string
    pub fn get_path(self) -> String {
        self.path
    }

    /// get callable
    pub fn get_callable(self) -> Option<Box<dyn CloneableFn<Output=ServerResponse>>> {
        self.f
    }

    /// get routes
    pub fn get_routes(self) -> Vec<Route> {
        self.routes
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
        (self.f.clone().unwrap())(request)
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#type, self.path)
    }
}

/// Create service route
pub fn service(path: &str, routes: Vec<Route>) -> Route {
    Route::service(path, routes)
}

/// Create GET route
pub fn get<F: CloneableFn<Output=ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Get, path, f)
}

/// Create POST route
pub fn post<F: CloneableFn<Output=ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Post, path, f)
}

/// Create PUT route
pub fn put<F: CloneableFn<Output=ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Put, path, f)
}

/// Create PATCH route
pub fn patch<F: CloneableFn<Output=ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Patch, path, f)
}

/// Create DELETE route
pub fn delete<F: CloneableFn<Output=ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Delete, path, f)
}
