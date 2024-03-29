use nova_web::prelude::*;

fn hello_world(_: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    res.status(HttpStatus::OK).body("Hello, World!".as_bytes())
}

fn not_found(_: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    res.status(HttpStatus::NotFound)
        .body("The path you're looking for is in another castle.".as_bytes())
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .fallback(not_found)
        .bind()
        .await
}
