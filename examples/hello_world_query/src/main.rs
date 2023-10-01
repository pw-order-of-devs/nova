use nova_web::errors::ServerError;
use nova_web::routing::ServerRouting;
use nova_web::server::Server;
use nova_web::types::request::HttpRequest;
use nova_web::types::response::{HttpResponse, ServerResponse};
use nova_web::types::status::HttpStatus;

fn hello_query(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let name = req.query("name")?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", name)))
}

fn hello_query_opt(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let name = req.query("name").unwrap_or("default".to_string());
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
