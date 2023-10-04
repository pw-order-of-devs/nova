use nova_web::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct RequestBody {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseBody {
    name: String,
}

fn hello_json(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.json()?;
    let response = ResponseBody { name: body.name };
    res.status(HttpStatus::OK)?.body_json(&response)
}

fn hello_form(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.form()?;
    let response = ResponseBody { name: body.name };
    res.status(HttpStatus::OK)?.body_json(&response)
}

fn hello_form_urlencoded(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.form_urlencoded()?;
    let response = ResponseBody { name: body.name };
    res.status(HttpStatus::OK)?.body_json(&response)
}

fn hello_xml(req: HttpRequest, res: HttpResponse) -> ServerResponse {
    let body: RequestBody = req.xml()?;
    let response = ResponseBody { name: body.name };
    res.status(HttpStatus::OK)?.body_xml(&response)
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    Server::create("0.0.0.0", 8181)
        .service(
            "/hello",
            vec![
                post("/json", hello_json),
                post("/form", hello_form),
                post("/form/urlencoded", hello_form_urlencoded),
                post("/xml", hello_xml),
            ]
            .into(),
        )
        .bind()
        .await
}
