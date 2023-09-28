use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;

/// Callable return type
pub type ServerResponse = Result<HttpResponse, ServerError>;

/// Base trait for route function
pub trait CloneableFn: FnMut(HttpRequest) -> ServerResponse + Send {
    /// clone route function
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<Output=ServerResponse>>
        where Self: 'a;
}

impl<F> CloneableFn for F
    where F: FnMut(HttpRequest) -> ServerResponse + Clone + Send {
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<Output=ServerResponse>>
        where Self: 'a {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + CloneableFn<Output=ServerResponse>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
