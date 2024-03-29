use nova_web::prelude::*;

fn hello_query(req: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    let name = req.query("name")?;
    res.status(HttpStatus::OK)
        .body(format!("Hello, {}!", name).as_bytes())
}

fn hello_query_opt(req: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    let name = req.query("name").unwrap_or("default".to_string());
    res.status(HttpStatus::OK)
        .body(format!("Hello, {}!", name).as_bytes())
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_query)
        .get("/hello/opt", hello_query_opt)
        .bind()
        .await
}
