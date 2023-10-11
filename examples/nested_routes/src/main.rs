use nova_web::prelude::*;

fn hello_world(_: &HttpRequest, res: &mut HttpResponse) -> ServerResponse {
    res.status(HttpStatus::OK).body("Hello, World!".as_bytes())
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .get("/hello", hello_world)
        .service(
            "/test",
            vec![
                get("/", hello_world),
                service("/1", vec![get("/", hello_world)].into()),
            ]
            .into(),
        )
        .bind()
        .await
}
