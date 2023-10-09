use nova_web::prelude::*;

#[derive(Clone, Debug)]
struct BasicAuthMiddleware {}

impl Middleware for BasicAuthMiddleware {
    fn request(&self, req: &mut HttpRequest) -> Result<(), ServerError> {
        match req.header("Authorization") {
            Some(auth) => BasicAuth::from_header(&auth)?.validate("user", "password"),
            None => Err(ServerError::Unauthorized),
        }
    }
}

fn hello_world(_: HttpRequest, mut res: HttpResponse) -> ServerResponse {
    res.status(HttpStatus::OK).body("Hello, World!".as_bytes())
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .middleware(BasicAuthMiddleware {})
        .middleware(LogRequestMiddleware {})
        .middleware(LogResponseMiddleware {})
        .get("/hello", hello_world)
        .bind()
        .await
}
