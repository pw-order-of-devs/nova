use std::str::FromStr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::protocol::Protocol;

use nova_router::callable::{CloneableFn, ServerResponse};
use nova_router::router::Router;

/// Nova server structure
#[derive(Clone, Debug)]
pub struct Server {
    host: String,
    port: u16,
    router: Router,
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
        Server { host: host.to_string(), port, router: Router::default(), }
    }

    /// Register new route
    pub fn route<F: CloneableFn<Output=ServerResponse> + 'static>(mut self, r#type: &str, path: &str, f: F) -> Self {
        self.router.register(r#type, path, f);
        self
    }

    /// Start Nova Server
    pub async fn bind(self) -> Result<(), ServerError> {
        tracing_subscriber::fmt::init();
        let listener = TcpListener::bind(&format!("{}:{}", self.host, self.port)).await?;
        tracing::info!("nova server listening at {}:{}", self.host, self.port);
        loop {
            let (mut stream, _) = listener.accept().await?;
            let router = self.router.clone();
            tokio::spawn(async move {
                match Self::handle_request(&mut stream).await {
                    Ok(request) => { let _ = Self::handle_response(&mut stream, request, router).await; },
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
            Err(e) => {
                tracing::error!("failed to read from socket; err = {:?}", e);
                Err(ServerError::ParseRequestError)
            }
        }
    }

    async fn handle_response(stream: &mut TcpStream, request: HttpRequest, router: Router) -> std::io::Result<()> {
        tracing::debug!("incoming request:\n{request}");
        match &mut router.match_route(request.get_route_path()) {
            Some(route) => {
                let request = request.update_path(&route.clone().get_path());
                match route.call(request) {
                    Ok(response) => stream.write_all(format!("{response}").as_bytes()).await,
                    Err(e) => Self::handle_error(stream, e).await,
                }
            },
            None => Self::handle_error(stream, ServerError::NotFound).await,
        }
    }

    async fn handle_error(stream: &mut TcpStream, error: ServerError) -> std::io::Result<()> {
        let response = HttpResponse::from_error(error, Protocol::default());
        tracing::debug!("server response:\n{response}");
        stream.write_all(format!("{response}").as_bytes()).await
    }
}
