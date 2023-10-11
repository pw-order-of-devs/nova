use nova_core::response::ServerResponse;
use nova_core::types::request_type::RequestType;

use crate::callable::CloneableFn;
use crate::routes::Routes;

/// Routes registration for server
pub trait ServerRouting
where
    Self: Clone,
{
    /// Register new route
    #[must_use]
    fn route<F: CloneableFn<Output = ServerResponse> + 'static>(
        &mut self,
        r#type: RequestType,
        path: &str,
        f: F,
    ) -> Self
    where
        Self: Sized;

    /// Register new service route
    #[must_use]
    fn service(&mut self, path: &str, routes: Routes) -> Self
    where
        Self: Sized;

    /// Register fallback route
    #[must_use]
    fn fallback<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, f: F) -> Self
    where
        Self: Sized;

    /// Register new get route
    #[must_use]
    fn get<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self
    where
        Self: Sized,
    {
        self.route(RequestType::Get, path, f)
    }

    /// Register new post route
    #[must_use]
    fn post<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self
    where
        Self: Sized,
    {
        self.route(RequestType::Post, path, f)
    }

    /// Register new put route
    #[must_use]
    fn put<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self
    where
        Self: Sized,
    {
        self.route(RequestType::Put, path, f)
    }

    /// Register new patch route
    #[must_use]
    fn patch<F: CloneableFn<Output = ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self
    where
        Self: Sized,
    {
        self.route(RequestType::Patch, path, f)
    }

    /// Register new delete route
    #[must_use]
    fn delete<F: CloneableFn<Output = ServerResponse> + 'static>(
        &mut self,
        path: &str,
        f: F,
    ) -> Self
    where
        Self: Sized,
    {
        self.route(RequestType::Delete, path, f)
    }
}
