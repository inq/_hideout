use std::path::Path;

#[derive(Default)]
pub struct AssetStore {
    inner: Vec<Asset>,
}

impl AssetStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P: AsRef<Path>>(
        &mut self,
        path: P,
        content_type: &str,
    ) -> Result<usize, failure::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        println!("{}", buf.len());
        self.inner.push(Asset {
            data: buf,
            content_type: content_type.to_string(),
        });
        Ok(self.inner.len() - 1)
    }

    pub fn serve(&self, id: usize) -> crate::http::Response {
        use crate::http::Response;
        Response::new_binary(200, &self.inner[id].data, &self.inner[id].content_type)
    }
}

pub struct Asset {
    data: Vec<u8>,
    content_type: String,
}
