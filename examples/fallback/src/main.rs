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

fn not_found(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    Ok(res
        .status(HttpStatus::NotFound)
        .body("The path you're looking for is in another castle."))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .fallback(not_found)
        .bind().await
}
