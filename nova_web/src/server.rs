use std::str::FromStr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use nova_core::errors::ServerError;
use nova_core::ext::request_ext::RequestExt;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;
use nova_core::types::protocol::Protocol;
use nova_router::router::Router;

/// Nova server structure
#[derive(Clone, Debug)]
pub struct Server {
    host: String,
    port: u16,
    pub(crate) router: Router,
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
        Server {
            host: host.to_string(),
            port,
            router: Router::default(),
            protocol: Protocol::default(),
        }
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
                    Ok(request) => {
                        let _ = Self::handle_response(&mut stream, request, router, self.protocol)
                            .await;
                    }
                    Err(e) => {
                        let _ = Self::handle_error(&mut stream, e).await;
                    }
                }
            });
        }
    }

    async fn handle_request(stream: &mut TcpStream) -> Result<HttpRequest, ServerError> {
        let mut buf = Vec::new();
        loop {
            let mut str = [0u8; 1024];
            let n = stream.read(&mut str).await;
            match n {
                Ok(n) if n == 0 => break,
                Ok(n) => {
                    buf.push(str.as_slice().to_vec());
                    if n < 1024 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(ServerError::ParseRequestError {
                        message: e.to_string(),
                    })
                }
            }
        }

        HttpRequest::from_str(&String::from_utf8(buf.into_iter().flatten().collect())?)
    }

    async fn handle_response(
        stream: &mut TcpStream,
        request: HttpRequest,
        router: Router,
        protocol: Protocol,
    ) -> std::io::Result<()> {
        let route_path = request.get_route_path();
        match &mut router.match_route(route_path.0, route_path.1, router.clone().get_fallback()) {
            Some((callable, path)) => match callable(
                request.update_path(path),
                HttpResponse::default().protocol(protocol),
            ) {
                Ok(response) => {
                    stream
                        .write_all(format!("{}", response.append_default_headers()).as_bytes())
                        .await
                }
                Err(e) => Self::handle_error(stream, e).await,
            },
            None => Self::handle_error(stream, ServerError::NotFound).await,
        }
    }

    async fn handle_error(stream: &mut TcpStream, error: ServerError) -> std::io::Result<()> {
        let response =
            HttpResponse::from_error(error, Protocol::default()).append_default_headers();
        stream.write_all(format!("{response}").as_bytes()).await
    }
}
