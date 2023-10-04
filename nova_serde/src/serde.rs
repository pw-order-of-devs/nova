pub use serde::{Deserialize, Serialize};

/// Define the `SerdeRequest` trait
pub trait SerdeRequest<S: for<'a> Deserialize<'a>> {
    /// Parsing Error type
    type Err;

    /// extract body for parsing
    fn get_serde_body(&self) -> String;

    /// parse serde error
    fn parse_error(_: impl std::error::Error) -> Self::Err;

    /// Parse json body to struct
    ///
    /// # Errors
    ///
    /// * `ServerError::ParseRequestError` - error while parsing the request body
    fn json(&self) -> Result<S, Self::Err> {
        match serde_json::from_str(&self.get_serde_body()) {
            Ok(body) => Ok(body),
            Err(err) => Err(Self::parse_error(err)),
        }
    }

    /// Parse form body to struct
    ///
    /// # Errors
    ///
    /// * `ServerError::ParseRequestError` - error while parsing the request body
    fn form(&self) -> Result<S, Self::Err> {
        let body = self.get_serde_body();
        let lines = body
            .split("\r\n")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let mut fields = vec![];

        for line in lines {
            if line.starts_with("--") {
                continue;
            }
            if line.to_lowercase().starts_with("content-disposition") {
                if !fields.is_empty() {
                    fields.push("&".to_string());
                }
                let mut name = line.to_string();
                name = name.replace("Content-Disposition: form-data; name=", "");
                name = name.replace('\"', "");
                fields.push(name);
            } else {
                fields.push(format!("={line}"));
            }
        }

        match serde_urlencoded::from_str(&fields.join("")) {
            Ok(body) => Ok(body),
            Err(err) => Err(Self::parse_error(err)),
        }
    }

    /// Parse x-www-form-urlencoded body to struct
    ///
    /// # Errors
    ///
    /// * `ServerError::ParseRequestError` - error while parsing the request body
    fn form_urlencoded(&self) -> Result<S, Self::Err> {
        match serde_urlencoded::from_str(&self.get_serde_body()) {
            Ok(body) => Ok(body),
            Err(err) => Err(Self::parse_error(err)),
        }
    }

    /// Parse xml body to struct
    ///
    /// # Errors
    ///
    /// * `ServerError::ParseRequestError` - error while parsing the request body
    fn xml(&self) -> Result<S, Self::Err> {
        match serde_xml_rs::from_str(&self.get_serde_body()) {
            Ok(body) => Ok(body),
            Err(err) => Err(Self::parse_error(err)),
        }
    }
}
