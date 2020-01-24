mod builder;

pub use builder::Builder;

pub type Handler = fn(&str) -> crate::http::Response;

pub struct Router {
    paths: Vec<(String, Handler)>,
}

impl Router {
    pub(self) fn from_builder(paths: Vec<(String, Handler)>) -> Self {
        Self { paths }
    }

    pub fn test_handle(&self, uri: &str) -> crate::http::Response {
        self.paths[0].1(uri)
    }
}
