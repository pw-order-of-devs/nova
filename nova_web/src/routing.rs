use nova_core::response::ServerResponse;
use nova_core::types::request_type::RequestType;

use nova_router::callable::CloneableFn;
use nova_router::route::Route;

pub use {
    nova_router::server_routing::ServerRouting,
    nova_router::route::{service, get, post, put, patch, delete},
};

use crate::server::Server;

impl ServerRouting for Server {
    fn route<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, r#type: RequestType, path: &str, f: F) -> Self {
        self.router.register(r#type, path, f);
        self.clone()
    }

    fn service(&mut self, path: &str, routes: Vec<Route>) -> Self where Self: Sized {
        routes.into_iter().for_each(|item| {
            let path = &format!("{path}{}", item.clone().get_path());
            if item.clone().get_callable().is_some() {
                self.route(item.clone().get_type(), path, item.get_callable().unwrap());
            } else {
                self.service(path, item.get_routes());
            }
        });
        self.clone()
    }

    fn fallback<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, f: F) -> Self where Self: Sized {
        self.router.register_fallback(f);
        self.clone()
    }
}
