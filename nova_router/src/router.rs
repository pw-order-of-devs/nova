use std::fmt::{Debug, Formatter};

use regex::Regex;

use nova_core::response::ServerResponse;
use nova_core::types::request_type::RequestType;

use crate::callable::{BoxedCallable, CloneableFn};
use crate::route::Route;
use crate::routes::Routes;

/// Nova Router structure
#[derive(Clone, Default)]
pub struct Router {
    routes: Routes,
    fallback: Option<BoxedCallable>,
}

impl Router {
    /// Register new route
    pub fn register<F: CloneableFn<Output = ServerResponse> + 'static>(
        &mut self,
        r#type: RequestType,
        path: &str,
        f: F,
    ) {
        let pattern = r"^/([a-zA-Z0-9_]+(/([a-zA-Z0-9_]+|\{[a-zA-Z0-9_]+\}))*/?)?$";
        if let Ok(regex) = Regex::new(pattern) {
            if regex.is_match(path) {
                self.routes.push(Route::route(r#type, path, f));
            }
        }
    }

    /// Register fallback route
    pub fn register_fallback<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, f: F) {
        self.fallback = Some(Box::new(f));
    }

    /// Get fallback route
    #[must_use]
    pub fn get_fallback(self) -> Option<BoxedCallable> {
        self.fallback
    }

    /// Find route for request
    #[must_use]
    pub fn match_route(
        &self,
        r#type: RequestType,
        path: &str,
        fallback: Option<BoxedCallable>,
    ) -> Option<(BoxedCallable, String)> {
        self.routes.find(r#type, path).map_or_else(
            || fallback.map(|f| (f, String::new())),
            |route| {
                route
                    .get_callable()
                    .map(|callable| (callable, route.get_path()))
            },
        )
    }
}

impl Debug for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.routes)
    }
}
