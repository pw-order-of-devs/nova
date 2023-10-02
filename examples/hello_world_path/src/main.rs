use nova_web::core::errors::ServerError;
use nova_web::routing::ServerRouting;
use nova_web::server::Server;
use nova_web::core::types::request::HttpRequest;
use nova_web::core::types::response::{HttpResponse, ServerResponse};
use nova_web::core::types::status::HttpStatus;

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
