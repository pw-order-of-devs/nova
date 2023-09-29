use std::fmt::{Debug, Formatter};
use regex::Regex;
use nova_core::types::request_type::RequestType;

use crate::callable::{BoxedCallable, CloneableFn, ServerResponse};
use crate::route::Route;

/// Nova Router structure
#[derive(Clone, Default)]
pub struct Router {
    routes: Vec<Route>,
    fallback: Option<BoxedCallable>,
}

impl Router {
    /// register new route
    pub fn register<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, r#type: RequestType, path: &str, f: F) {
        if self.routes.iter().any(|r| r.clone().get_path() == path) {
            return;
        }
        let pattern = r"^/([a-zA-Z0-9_]+(/([a-zA-Z0-9_]+|\{[a-zA-Z0-9_]+\}))*/?)?$";
        if !Regex::new(pattern).unwrap().is_match(path) {
            return;
        }
        self.routes.push(Route::route(r#type, path, f));
    }

    /// register fallback route
    pub fn register_fallback<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, f: F) {
        self.fallback = Some(Box::new(f))
    }

    /// get fallback route
    pub fn get_fallback(self) -> Option<BoxedCallable> {
        self.fallback
    }

    /// find route for request
    pub fn match_route(&self, route: (RequestType, String), fallback: Option<BoxedCallable>) -> Option<(BoxedCallable, String)> {
        if let Some(route) = self.routes.clone().into_iter().find(|r| r.matches(route.0, &route.1)) {
            Some((route.clone().get_callable().unwrap(), route.get_path()))
        } else {
            fallback.map(|f| (f, "".to_string()))
        }
    }
}

impl Debug for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.routes)
    }
}
