use nova_core::errors::ServerError;
use nova_web::server::Server;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .bind().await
}
