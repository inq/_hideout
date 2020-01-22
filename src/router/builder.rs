use super::Router;

pub struct Builder {
    paths: Vec<(String, &'static dyn Fn(&str) -> ())>,
}

impl Builder {
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn add_path<F>(&mut self, path: &str, handler: &'static F) -> &mut Self
    where
        F: Fn(&str) -> (),
    {
        self.paths.push((path.to_string(), handler));
        self
    }

    pub fn build(self) -> Router {
        Router::from_builder(self.paths)
    }
}
