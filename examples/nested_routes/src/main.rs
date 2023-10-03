use nova_web::core::errors::ServerError;
use nova_web::core::types::request::HttpRequest;
use nova_web::core::types::response::{HttpResponse, ServerResponse};
use nova_web::core::types::status::HttpStatus;
use nova_web::routing::{get, service, ServerRouting};
use nova_web::server::Server;

fn hello_world(_: HttpRequest, res: HttpResponse) -> ServerResponse {
    Ok(res.status(HttpStatus::OK).body("Hello, World!"))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .service(
            "/test",
            vec![
                get("/", hello_world),
                service("/1", vec![get("/", hello_world)]),
            ],
        )
        .bind()
        .await
}
