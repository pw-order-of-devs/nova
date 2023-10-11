use crate::ext::hash_map_ext::HashMapExt;
use crate::request::HttpRequest;
use crate::types::path::Path;
use crate::types::request_type::RequestType;

/// `HttpRequest` extension trait
pub trait RequestExt {
    /// Extract route details
    fn get_route_path(&self) -> (RequestType, String);

    /// Update path map from route
    #[must_use]
    fn update_path(self, route: &str) -> Self;
}

impl RequestExt for HttpRequest {
    fn get_route_path(&self) -> (RequestType, String) {
        (self.clone().r#type, self.clone().target)
    }

    fn update_path(mut self, route: &str) -> Self {
        let self_segments = self.target.split('/').filter(|s| !s.is_empty());
        let segments = route
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let inner = self_segments
            .zip(segments)
            .filter(|(_, t)| (t.starts_with('{') && t.ends_with('}')))
            .map(|(s, t)| (t[1..t.len() - 1].to_string(), s.to_string()))
            .collect();
        self.path = Path::new(inner);
        self
    }
}
