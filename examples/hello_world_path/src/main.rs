use nova_web::prelude::*;

fn hello_path(req: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    let path = req.path("name")?;
    res.status(HttpStatus::OK)
        .body(format!("Hello, {}!", path).as_bytes())
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello/{name}", hello_path)
        .bind()
        .await
}
