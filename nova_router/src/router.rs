use regex::Regex;
use nova_core::types::request_type::RequestType;

use crate::callable::{CloneableFn, ServerResponse};
use crate::route::Route;

/// Nova Router structure
#[derive(Clone, Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
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

    /// find route for request
    pub fn match_route(self, route: (RequestType, String)) -> Option<Route> {
        self.routes.into_iter().find(|r| r.matches(route.0, &route.1))
    }
}
