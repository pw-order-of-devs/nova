use serde::Serialize;

/// Define the `SerdeResponse` trait
pub trait SerdeResponse<S: Serialize> {
    /// Parsing Error type
    type Err;

    /// set body for response
    #[must_use]
    fn with_body(&self, body: &[u8]) -> Self;

    /// set header for response
    #[must_use]
    fn with_header(&self, key: &str, value: &str) -> Self;

    /// parse serde error
    fn parse_error(_: impl std::error::Error) -> Self::Err;

    /// Parse body struct to json
    ///
    /// # Errors
    ///
    /// * `ServerError::ParseError` - error while parsing the response body
    fn body_json(&self, item: S) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        match serde_json::to_string(&item) {
            Ok(body) => Ok(self
                .with_header("Content-Type", "application/json")
                .with_body(body.as_bytes())),
            Err(err) => Err(Self::parse_error(err)),
        }
    }

    /// Parse body struct to xml
    ///
    /// # Errors
    ///
    /// * `ServerError::ParseError` - error while parsing the response body
    fn body_xml(&self, item: S) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        match serde_xml_rs::to_string(&item) {
            Ok(body) => Ok(self
                .with_header("Content-Type", "application/xml")
                .with_body(body.as_bytes())),
            Err(err) => Err(Self::parse_error(err)),
        }
    }
}
