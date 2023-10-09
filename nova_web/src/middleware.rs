use std::fmt::Debug;
pub use {
    nova_middleware::r#impl::logging::*,
    nova_middleware::middleware::{BoxedMiddleware, Middleware},
    nova_middleware::server_middleware::ServerMiddleware,
};

use crate::server::Server;

impl ServerMiddleware for Server {
    fn middleware<T: Middleware + Clone + Debug + Send + Sync + 'static>(
        &mut self,
        middleware: T,
    ) -> Self
    where
        Self: Sized,
    {
        self.middleware.register(Box::new(middleware));
        self.clone()
    }
}
