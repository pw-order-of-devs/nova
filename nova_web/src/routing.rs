use nova_core::response::ServerResponse;
use nova_core::types::request_type::RequestType;
use nova_router::callable::CloneableFn;
use nova_router::routes::Routes;
pub use {
    nova_router::route::{delete, get, patch, post, put, service},
    nova_router::server_routing::ServerRouting,
};

use crate::server::Server;

impl ServerRouting for Server {
    fn route<F: CloneableFn<Output = ServerResponse> + 'static>(
        &mut self,
        r#type: RequestType,
        path: &str,
        f: F,
    ) -> Self {
        self.router.register(r#type, path, f);
        self.clone()
    }

    fn service(&mut self, path: &str, routes: Routes) -> Self
    where
        Self: Sized,
    {
        routes.for_each(|item| {
            let path = &format!("{path}{}", item.get_path());
            if item.get_callable().is_some() {
                let _ = self.route(item.get_type(), path, item.get_callable().unwrap());
            } else {
                let _ = self.service(path, item.get_routes());
            }
        });
        self.clone()
    }

    fn fallback<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, f: F) -> Self
    where
        Self: Sized,
    {
        self.router.register_fallback(f);
        self.clone()
    }
}
