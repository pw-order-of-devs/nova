use nova_core::request::HttpRequest;
use nova_core::response::{HttpResponse, ServerResponse};

/// Definition of Callable type
pub type BoxedCallable = Box<dyn CloneableFn<Output = ServerResponse> + 'static>;

/// Base trait for route function
pub trait CloneableFn:
    FnMut(&HttpRequest, &mut HttpResponse) -> ServerResponse + Sync + Send
{
    /// clone route function
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<Output = ServerResponse>>
    where
        Self: 'a;
}

impl<F> CloneableFn for F
where
    F: FnMut(&HttpRequest, &mut HttpResponse) -> ServerResponse + Clone + Sync + Send,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + CloneableFn<Output = ServerResponse>>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + CloneableFn<Output = ServerResponse>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
