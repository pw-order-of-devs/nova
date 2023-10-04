use nova_web::prelude::*;

fn hello_world(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    res.status(HttpStatus::OK).body("Hello, World!")
}

fn error_page(_: HttpRequest, _: HttpResponse) -> ServerResponse {
    Err(ServerError::InternalError)
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .get("/error", error_page)
        .bind()
        .await
}
