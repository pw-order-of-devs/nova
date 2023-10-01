use nova_web::core::errors::ServerError;
use nova_web::routing::ServerRouting;
use nova_web::server::Server;
use nova_web::core::types::request::HttpRequest;
use nova_web::core::types::response::{HttpResponse, ServerResponse};
use nova_web::core::types::status::HttpStatus;

fn hello_world(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    Ok(res
        .status(HttpStatus::OK)
        .body("Hello, World!"))
}

fn error_page(_: HttpRequest, _: HttpResponse) -> ServerResponse {
    Err(ServerError::InternalError)
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .get("/error", error_page)
        .bind().await
}
