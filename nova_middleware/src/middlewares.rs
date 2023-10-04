use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::middleware::BoxedMiddleware;

/// Nova Middleware wrapper
#[derive(Clone, Debug, Default)]
pub struct Middlewares {
    inner: HashMap<String, BoxedMiddleware>,
}

impl Middlewares {
    /// Register new Middleware
    pub fn register(&mut self, middle: BoxedMiddleware) {
        self.inner.insert(uuid::Uuid::new_v4().to_string(), middle);
    }

    /// Call Request middleware
    ///
    /// # Errors
    ///
    /// * if calling middleware returns an error, this error is returned
    pub fn call_for_req(&self, req: &mut HttpRequest) -> Result<(), ServerError> {
        let binding = self
            .inner
            .values()
            .map(|boxed| boxed.request(req))
            .collect::<Vec<Result<(), _>>>();
        let error = binding.iter().find(|item| item.is_err());
        error.map_or(Ok(()), Clone::clone)
    }

    /// Call Response middleware
    ///
    /// # Errors
    ///
    /// * if calling middleware returns an error, this error is returned
    pub fn call_for_res(&self, res: &mut HttpResponse) -> Result<(), ServerError> {
        let binding = self
            .inner
            .values()
            .map(|boxed| boxed.response(res))
            .collect::<Vec<Result<(), _>>>();
        let error = binding.iter().find(|item| item.is_err());
        error.map_or(Ok(()), Clone::clone)
    }
}
