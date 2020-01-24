use super::{Handler, Router};

pub struct Builder {
    paths: Vec<(String, Handler)>,
}

impl Builder {
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn add_path(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.paths.push((path.to_string(), handler));
        self
    }

    pub fn build(self) -> Router {
        Router::from_builder(self.paths)
    }
}
