use std::fmt::Debug;

use crate::middleware::Middleware;

/// Middleware registration for server
pub trait ServerMiddleware
where
    Self: Clone,
{
    /// Register new middleware
    #[must_use]
    fn middleware<T: Middleware + Clone + Debug + Send + Sync + 'static>(
        &mut self,
        middleware: T,
    ) -> Self
    where
        Self: Sized;
}
