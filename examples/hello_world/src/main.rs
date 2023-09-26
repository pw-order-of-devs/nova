use nova_core::errors::NovaError;
use nova_web::server::Server;

#[tokio::main]
async fn main() -> Result<(), NovaError> {
    Server::create("0.0.0.0", 8181)
        .bind().await
}
