use nova_web::middleware::ServerMiddleware;
use nova_web::prelude::*;

#[derive(Clone, Debug)]
struct LogRequestMiddleware {}

impl Middleware for LogRequestMiddleware {
    fn request(&self, req: &mut HttpRequest) -> Result<(), ServerError> {
        eprintln!("{req}");
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct LogResponseMiddleware {}

impl Middleware for LogResponseMiddleware {
    fn response(&self, res: &mut HttpResponse) -> Result<(), ServerError> {
        eprintln!("{res}");
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct AuthorizeMiddleware {}

impl Middleware for AuthorizeMiddleware {
    fn request(&self, req: &mut HttpRequest) -> Result<(), ServerError> {
        match req.header("Authorization") {
            Some(auth) => BasicAuth::from_header(&auth)?.validate("user", "password"),
            None => Err(ServerError::Unauthorized),
        }
    }
}

fn hello_world(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    res.status(HttpStatus::OK).body("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .middleware(LogRequestMiddleware {})
        .middleware(LogResponseMiddleware {})
        .middleware(AuthorizeMiddleware {})
        .get("/hello", hello_world)
        .bind()
        .await
}
