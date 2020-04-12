use hideout::{http, util::AssetStore};
pub(super) struct Assets {}

lazy_static::lazy_static! {
    static ref STORE: AssetStore = {
        let mut asset_store = AssetStore::new();
        asset_store.add("assets/raleway-light.woff", "font/woff").unwrap();
        asset_store
    };
}

impl Assets {
    pub(super) async fn serve_inner(
        request: http::Request,
        _payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        match (request.uri().nth_path(idx), request.uri().nth_path(idx + 1)) {
            (Some(resource), None) => {
                if let Some(res) = STORE.serve(resource) {
                    return Ok(res);
                }
            }
            _ => (),
        }
        Err(http::Error::NotFound {
            uri: request.uri().as_str().to_string(),
        })
    }
}
