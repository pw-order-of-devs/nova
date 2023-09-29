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
