use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponseBuilder;
use nova_core::types::status::HttpStatus;

use nova_router::callable::ServerResponse;

use nova_web::server::Server;

fn hello_query(req: HttpRequest) -> ServerResponse {
    let query = req.query("name")?;
    let response = HttpResponseBuilder::new()
        .status(HttpStatus::OK)
        .body(&format!("Hello World, {}!", query))
        .build();
    Ok(response)
}

fn hello_query_opt(req: HttpRequest) -> ServerResponse {
    let name = match req.query("name") {
        Ok(name) => name,
        Err(_) => "default".to_string(),
    };
    let response = HttpResponseBuilder::new()
        .status(HttpStatus::OK)
        .body(&format!("Hello World, {}!", name))
        .build();
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .route("GET", "/hello", hello_query)
        .route("GET", "/hello/opt", hello_query_opt)
        .bind().await
}
