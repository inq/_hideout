use std::collections::HashMap;
use std::path::Path;

pub struct Asset {
    data: Vec<u8>,
    content_type: String,
}

#[derive(Default)]
pub struct AssetStore {
    inner: HashMap<String, Asset>,
}

impl AssetStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P: AsRef<Path>>(
        &mut self,
        path: P,
        content_type: &str,
    ) -> Result<(), failure::Error> {
        // TODO: Remove unwraps
        let key = path
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let buf = std::fs::read(path)?;
        self.inner.insert(
            key,
            Asset {
                data: buf,
                content_type: content_type.to_string(),
            },
        );
        Ok(())
    }

    pub fn serve(&self, key: &str) -> Option<crate::http::Response> {
        self.inner.get(key).map(|asset| {
            use crate::http::Response;

            Response::new_binary(200, &asset.data, &asset.content_type)
        })
    }
}
