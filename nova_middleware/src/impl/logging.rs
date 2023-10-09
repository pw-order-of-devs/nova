use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;

use crate::middleware::Middleware;

/// Middleware for logging HTTP requests
#[derive(Clone, Debug)]
pub struct LogRequestMiddleware {}

impl Middleware for LogRequestMiddleware {
    fn request(&self, req: &mut HttpRequest) -> Result<(), ServerError> {
        eprintln!("{req}");
        Ok(())
    }
}

/// Middleware for logging HTTP responses
#[derive(Clone, Debug)]
pub struct LogResponseMiddleware {}

impl Middleware for LogResponseMiddleware {
    fn response(&self, _: &HttpRequest, res: &mut HttpResponse) -> Result<(), ServerError> {
        eprintln!("{res}");
        Ok(())
    }
}
