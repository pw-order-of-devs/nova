use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::status::HttpStatus;

use nova_router::callable::ServerResponse;
use nova_router::server_routing::ServerRouting;

use nova_web::server::Server;

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
