use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::status::HttpStatus;
use nova_router::callable::ServerResponse;
use nova_web::server::Server;

fn hello_world(_: HttpRequest) -> ServerResponse {
    Ok(HttpResponse::build(HttpStatus::OK, "Hello World!", Default::default(), "HTTP/1.1"))
}

fn error_page(_: HttpRequest) -> ServerResponse {
    Err(ServerError::InternalError)
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .route("GET", "/hello", hello_world)
        .route("GET", "/error", error_page)
        .bind().await
}
