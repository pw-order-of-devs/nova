use nova_serde::serde::{Deserialize, SerdeRequest};

use crate::errors::ServerError;
use crate::request::HttpRequest;

impl <S: for<'a> Deserialize<'a>> SerdeRequest<S> for HttpRequest {
    type Err = ServerError;

    fn get_serde_body(&self) -> String {
        self.body_string()
    }

    fn parse_error(err: impl std::error::Error) -> Self::Err {
        ServerError::ParseRequestError { message: err.to_string() }
    }
}
