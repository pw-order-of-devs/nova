use nova_core::types::request_type::RequestType;

use crate::callable::{CloneableFn, ServerResponse};

/// Routes registration for server
pub trait ServerRouting where Self: Clone {
    /// Register new route
    fn route<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, r#type: RequestType, path: &str, f: F) -> Self where Self: Sized;

    /// Register new get route
    fn get<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self where Self: Sized {
        self.route(RequestType::Get, path, f)
    }

    /// Register new post route
    fn post<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self where Self: Sized {
        self.route(RequestType::Post, path, f)
    }

    /// Register new put route
    fn put<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self where Self: Sized {
        self.route(RequestType::Put, path, f)
    }

    /// Register new patch route
    fn patch<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self where Self: Sized {
        self.route(RequestType::Patch, path, f)
    }

    /// Register new delete route
    fn delete<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, path: &str, f: F) -> Self where Self: Sized {
        self.route(RequestType::Delete, path, f)
    }
}
