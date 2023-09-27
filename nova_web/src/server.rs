use std::str::FromStr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use nova_core::errors::ServerError;
use nova_core::request::HttpRequest;
use nova_core::response::HttpResponse;

/// Nova server structure
#[derive(Debug)]
pub struct Server {
    host: String,
    port: u16,
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
        Server { host: host.to_string(), port }
    }

    /// Start Nova Server
    pub async fn bind(&self) -> Result<(), ServerError> {
        tracing_subscriber::fmt::init();
        let listener = TcpListener::bind(&format!("{}:{}", self.host, self.port)).await?;
        tracing::info!("nova server listening at {}:{}", self.host, self.port);
        loop {
            let (mut stream, _) = listener.accept().await?;
            tokio::spawn(async move {
                match Self::handle_request(&mut stream).await {
                    Ok(request) => {
                        tracing::info!("incoming request:\n{request}");
                        let _ = Self::handle_response(&mut stream).await;
                    },
                    Err(e) => { let _ = Self::handle_error(&mut stream, e).await; },
                }
            });
        }
    }

    async fn handle_request(stream: &mut TcpStream) -> Result<HttpRequest, ServerError> {
        let mut str = [0u8; 8192];
        match stream.read(&mut str).await {
            Ok(n) if n == 0 => Err(ServerError::EmptyRequest),
            Ok(_) => HttpRequest::from_str(&String::from_utf8(str.to_vec()).unwrap()),
            Err(e) => {
                tracing::error!("failed to read from socket; err = {:?}", e);
                Err(ServerError::ParseRequestError)
            }
        }
    }

    async fn handle_response(stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!").await
    }

    async fn handle_error(stream: &mut TcpStream, error: ServerError) -> std::io::Result<()> {
        let response = HttpResponse::from_error(error, "HTTP/1.1");
        tracing::debug!("server response:\n{response}");
        stream.write_all(format!("{response}").as_bytes()).await
    }
}
