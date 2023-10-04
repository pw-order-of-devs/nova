use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use nova_core::response::ServerResponse;
use nova_core::types::request_type::RequestType;

use crate::callable::{BoxedCallable, CloneableFn};
use crate::routes::Routes;

/// Nova Route structure
#[derive(Clone)]
pub struct Route {
    r#type: RequestType,
    path: String,
    f: Option<BoxedCallable>,
    routes: Routes,
}

#[allow(clippy::self_named_constructors)]
impl Route {
    /// create new service route
    pub fn service(path: &str, routes: Routes) -> Self {
        Self {
            r#type: RequestType::Get,
            path: path.to_string(),
            f: None,
            routes,
        }
    }

    /// create new route
    pub fn route<F: CloneableFn<Output = ServerResponse> + 'static>(
        r#type: RequestType,
        path: &str,
        f: F,
    ) -> Self {
        Self {
            r#type,
            path: path.to_string(),
            f: Some(Box::new(f)),
            routes: Routes::default(),
        }
    }

    /// get request type
    pub fn get_type(&self) -> RequestType {
        self.r#type
    }

    /// get path string
    pub fn get_path(&self) -> String {
        self.clone().path
    }

    /// get callable
    pub fn get_callable(&self) -> Option<BoxedCallable> {
        self.clone().f
    }

    /// get routes
    pub fn get_routes(&self) -> Routes {
        self.clone().routes
    }

    /// check if route matches predicate
    pub fn matches(&self, r#type: RequestType, path: &str) -> bool {
        if self.r#type != r#type {
            return false;
        }
        let self_segments = self
            .path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        if self_segments.len() != segments.len() {
            return false;
        }
        self_segments
            .into_iter()
            .zip(segments)
            .all(|(s, t)| s == t || (s.starts_with('{') && s.ends_with('}')))
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#type, self.path)
    }
}

impl PartialEq<Self> for Route {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.path == other.path
    }
}

impl Eq for Route {}

impl Hash for Route {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
        self.path.hash(state);
    }
}

/// Create service route
pub fn service(path: &str, routes: Routes) -> Route {
    Route::service(path, routes)
}

/// Create GET route
pub fn get<F: CloneableFn<Output = ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Get, path, f)
}

/// Create POST route
pub fn post<F: CloneableFn<Output = ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Post, path, f)
}

/// Create PUT route
pub fn put<F: CloneableFn<Output = ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Put, path, f)
}

/// Create PATCH route
pub fn patch<F: CloneableFn<Output = ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Patch, path, f)
}

/// Create DELETE route
pub fn delete<F: CloneableFn<Output = ServerResponse> + 'static>(path: &str, f: F) -> Route {
    Route::route(RequestType::Delete, path, f)
}
