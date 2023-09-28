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

    /// find route by path
    pub fn find_route_by_path(self, r#type: &str, path: &str) -> Option<Route> {
        self.routes.into_iter().find(|r| r.matches(r#type, path))
    }
}
