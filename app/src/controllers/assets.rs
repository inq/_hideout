use hideout::{
    http::{Request, Response},
    util::AssetStore,
};
pub(super) struct Assets {}

lazy_static::lazy_static! {
    static ref STORE: AssetStore = {
        let mut asset_store = AssetStore::new();
        asset_store.add("assets/raleway-light.woff", "font/woff").unwrap();
        asset_store
    };
}

impl Assets {
    pub(super) async fn serve_inner(request: Request, _payload: &[u8], idx: usize) -> Response {
        match (request.uri().nth_path(idx), request.uri().nth_path(idx + 1)) {
            (Some(resource), None) => STORE
                .serve(resource)
                .unwrap_or_else(|| crate::handlers::not_found(request.uri().as_str())),
            _ => crate::handlers::not_found(request.uri().as_str()),
        }
    }
}
