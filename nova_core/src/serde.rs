use nova_serde::request::SerdeRequest;
use nova_serde::response::SerdeResponse;
use nova_serde::{Deserialize, Serialize};

use crate::errors::ServerError;
use crate::request::HttpRequest;
use crate::response::{HttpResponse, HttpResponseData};

impl<S: for<'a> Deserialize<'a>> SerdeRequest<S> for HttpRequest {
    type Err = ServerError;

    fn get_serde_body(&self) -> String {
        self.body_string()
    }

    fn parse_error(err: impl std::error::Error) -> Self::Err {
        ServerError::ParseError {
            message: err.to_string(),
        }
    }
}

impl<S: Serialize> SerdeResponse<S> for HttpResponse {
    type Err = ServerError;

    fn with_body(&self, body: &[u8]) -> Self {
        self.clone().body(body).map_or_else(|_| self.clone(), |s| s)
    }

    fn parse_error(err: impl std::error::Error) -> Self::Err {
        ServerError::ParseError {
            message: err.to_string(),
        }
    }
}
