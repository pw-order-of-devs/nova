use nova_web::prelude::*;

fn hello_world(_: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    res.status(HttpStatus::OK).body("Hello, World!".as_bytes())
}

fn error_page(_: &HttpRequest, _: &mut HttpResponse) -> ServerResponse {
    Err(ServerError::InternalError {
        message: "error happened".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .get("/error", error_page)
        .bind()
        .await
}
