use nova_web::errors::ServerError;
use nova_web::routing::{get, ServerRouting, service};
use nova_web::server::Server;
use nova_web::types::request::HttpRequest;
use nova_web::types::response::{HttpResponse, ServerResponse};
use nova_web::types::status::HttpStatus;

fn hello_world(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    Ok(res
        .status(HttpStatus::OK)
        .body("Hello, World!"))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .service("/test", vec![
            get("/", hello_world),
            service("/1", vec![
                get("/", hello_world)
            ])
        ])
        .bind().await
}
