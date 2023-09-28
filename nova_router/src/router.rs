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
    pub fn register<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, r#type: &str, path: &str, f: F) {
        self.routes.push(Route::new(r#type, path, f));
    }

    /// find route for request
    pub fn match_route(self, route: (RequestType, String)) -> Option<Route> {
        let (r#type, path) = (route.0.to_string(), route.1);
        self.routes.into_iter().find(|r| r.matches(&r#type, &path))
    }
}
