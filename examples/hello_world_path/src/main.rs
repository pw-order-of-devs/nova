use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::status::HttpStatus;

use nova_router::callable::ServerResponse;
use nova_router::server_routing::ServerRouting;

use nova_web::server::Server;

fn hello_path(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let path = req.path("name")?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", path)))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello/{name}", hello_path)
        .bind().await
}
