use std::str::FromStr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::protocol::Protocol;
use nova_core::types::request_type::RequestType;

use nova_router::callable::{CloneableFn, ServerResponse};
use nova_router::route::Route;
use nova_router::router::Router;
use nova_router::server_routing::ServerRouting;

/// Nova server structure
#[derive(Clone, Debug)]
pub struct Server {
    host: String,
    port: u16,
    router: Router,
    protocol: Protocol,
}

/// Nova Server implementation
impl Server {

    /// Create an instance of Nova Server
    ///
    /// # Arguments
    /// * `host` - server address, like '0.0.0.0'
    /// * `port` - port to listen on, like '8080'
    ///
    pub fn create(host: &str, port: u16) -> Self {
        Server { host: host.to_string(), port, router: Router::default(), protocol: Protocol::default(), }
    }

    /// Set HTTP protocol used by Nova Server
    ///
    /// # Arguments
    /// * `protocol` - HTTP protocol to use: {HTTP/1.1, HTTP/2}
    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Start Nova Server
    pub async fn bind(self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(&format!("{}:{}", self.host, self.port)).await?;
        loop {
            let (mut stream, _) = listener.accept().await?;
            let router = self.router.clone();
            tokio::spawn(async move {
                match Self::handle_request(&mut stream).await {
                    Ok(request) => { let _ = Self::handle_response(&mut stream, request, router, self.protocol).await; },
                    Err(e) => { let _ = Self::handle_error(&mut stream, e).await; },
                }
            });
        }
    }

    async fn handle_request(stream: &mut TcpStream) -> Result<HttpRequest, ServerError> {
        let mut str = [0u8; 8192];
        match stream.read(&mut str).await {
            Ok(n) if n == 0 => Err(ServerError::EmptyRequest),
            Ok(_) => HttpRequest::from_str(&String::from_utf8(str.as_slice().to_vec())?),
            Err(e) => Err(ServerError::ParseRequestError { message: e.to_string() })
        }
    }

    async fn handle_response(stream: &mut TcpStream, request: HttpRequest, router: Router, protocol: Protocol) -> std::io::Result<()> {
        let fallback = router.clone().get_fallback();
        match &mut router.match_route(request.get_route_path()) {
            Some(route) => {
                let response = HttpResponse::default()
                    .protocol(protocol);
                let request = request.update_path(&route.clone().get_path());
                match route.call(request, response) {
                    Ok(response) => stream.write_all(format!("{response}").as_bytes()).await,
                    Err(e) => Self::handle_error(stream, e).await,
                }
            },
            None => Self::handle_error(stream, ServerError::NotFound).await,
        }
    }

    async fn handle_error(stream: &mut TcpStream, error: ServerError) -> std::io::Result<()> {
        let response = HttpResponse::from_error(error, Protocol::default())
            .append_default_headers();
        stream.write_all(format!("{response}").as_bytes()).await
    }
}

impl ServerRouting for Server {
    fn route<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, r#type: RequestType, path: &str, f: F) -> Self {
        self.router.register(r#type, path, f);
        self.clone()
    }

    fn service(&mut self, path: &str, routes: Vec<Route>) -> Self where Self: Sized {
        routes.into_iter().for_each(|item| {
            let path = &format!("{path}{}", item.clone().get_path());
            if item.clone().get_callable().is_some() {
                self.route(item.clone().get_type(), path, item.get_callable().unwrap());
            } else {
                self.service(path, item.get_routes());
            }
        });
        self.clone()
    }

    fn fallback<F: CloneableFn<Output=ServerResponse> + 'static>(&mut self, f: F) -> Self where Self: Sized {
        self.router.register_fallback(f);
        self.clone()
    }
}
