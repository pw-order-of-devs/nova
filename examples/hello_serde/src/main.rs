use nova_web::core::errors::ServerError;
use nova_web::core::types::request::HttpRequest;
use nova_web::core::types::response::{HttpResponse, ServerResponse};
use nova_web::core::types::status::HttpStatus;
use nova_web::routing::ServerRouting;
use nova_web::serde::{Deserialize, SerdeRequest, Serialize};
use nova_web::server::Server;

#[derive(Debug, Deserialize, Serialize)]
struct RequestBody {
    id: String,
    name: String,
}

fn hello_json(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.json()?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", body.name)))
}

fn hello_form(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.form()?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", body.name)))
}

fn hello_form_urlencoded(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.form_urlencoded()?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", body.name)))
}

fn hello_xml(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.xml()?;
    Ok(res
        .status(HttpStatus::OK)
        .body(&format!("Hello, {}!", body.name)))
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .post("/hello/json", hello_json)
        .post("/hello/form", hello_form)
        .post("/hello/form/urlencoded", hello_form_urlencoded)
        .post("/hello/xml", hello_xml)
        .bind()
        .await
}
