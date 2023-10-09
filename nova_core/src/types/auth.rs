use base64::{engine::general_purpose::STANDARD, Engine};

use crate::errors::ServerError;

/// Basic Auth struct
#[derive(Clone, Debug)]
pub struct BasicAuth {
    user: String,
    password: String,
}

impl BasicAuth {
    /// Create `BasicAuth` instance
    ///
    /// # Errors
    ///
    /// * if auth header is invalid, `ServerError::InternalError` is returned
    pub fn from_header(header: &str) -> Result<Self, ServerError> {
        let mut base64 = header.to_string();
        if header.starts_with("Basic ") {
            base64 = base64[6..].to_string();
        }
        match STANDARD.decode(base64) {
            Ok(creds) => {
                let creds = String::from_utf8(creds)?;
                let creds = creds.split(':').collect::<Vec<&str>>();
                if creds.len() == 2 {
                    Ok(Self {
                        user: creds[0].to_string(),
                        password: creds[1].to_string(),
                    })
                } else {
                    Err(ServerError::InternalError {
                        message: "Basic Auth header is invalid".to_string(),
                    })
                }
            }
            Err(err) => Err(ServerError::InternalError { message: err.to_string() }),
        }
    }

    /// Validate credentials match
    ///
    /// # Errors
    ///
    /// * if credentials don't match, `ServerError::Unauthorized` is returned
    pub fn validate(&self, user: &str, password: &str) -> Result<(), ServerError> {
        if self.user == user && self.password == password {
            Ok(())
        } else {
            Err(ServerError::Unauthorized)
        }
    }
}
