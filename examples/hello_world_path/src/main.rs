use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponseBuilder;
use nova_core::types::status::HttpStatus;

use nova_router::callable::ServerResponse;
use nova_router::server_routing::ServerRouting;

use nova_web::server::Server;

fn hello_path(req: HttpRequest) -> ServerResponse {
    let path = req.path("name")?;
    let response = HttpResponseBuilder::new()
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", path))
        .build();
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello/{name}", hello_path)
        .bind().await
}
