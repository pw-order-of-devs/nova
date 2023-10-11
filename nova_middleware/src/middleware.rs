use nova_core::errors::ServerError;
use std::fmt::Debug;

use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;

/// Definition of `BoxedMiddleware` type wrapper
pub type BoxedMiddleware = Box<dyn CloneableMiddleware + 'static>;

/// Nova Middleware trait
pub trait Middleware {
    /// Middleware Request operation
    ///
    /// # Errors
    ///
    /// * if calling middleware returns an error, this error is returned
    fn request(&self, _: &mut HttpRequest) -> Result<(), ServerError> {
        Ok(())
    }

    /// Middleware Response operation
    ///
    /// # Errors
    ///
    /// * if calling middleware returns an error, this error is returned
    fn response(&self, _: &HttpRequest, _: &mut HttpResponse) -> Result<(), ServerError> {
        Ok(())
    }
}

/// Base trait for Middleware
pub trait CloneableMiddleware: Middleware + Debug + Sync + Send {
    /// clone middleware
    fn clone_box<'a>(&self) -> Box<dyn CloneableMiddleware + 'static>
    where
        Self: 'a;
}

impl<F: ?Sized + 'static> CloneableMiddleware for F
where
    F: Middleware + Clone + Debug + Sync + Send,
{
    fn clone_box<'a>(&self) -> Box<dyn CloneableMiddleware + 'static>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneableMiddleware> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
