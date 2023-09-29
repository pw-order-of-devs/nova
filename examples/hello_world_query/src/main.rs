use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::status::HttpStatus;

use nova_router::callable::ServerResponse;
use nova_router::server_routing::ServerRouting;

use nova_web::server::Server;

fn hello_query(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let name = req.query("name")?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", name)))
}

fn hello_query_opt(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let name = match req.query("name") {
        Ok(name) => name,
        Err(_) => "default".to_string(),
    };
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", name)))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_query)
        .get("/hello/opt", hello_query_opt)
        .bind().await
}
