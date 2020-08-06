use hideout::{http, util::AssetStore};
pub(super) struct Assets {}

impl Images {
    pub(super) async fn serve_inner(
        context: crate::Context,
        _payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if let (Some(resource), None) = (
            context.request.uri().nth_path(idx),
            context.request.uri().nth_path(idx + 1),
        ) {
            if let Some(res) = STORE.serve(&resource) {
                return Ok(res);
            }
        }
        Err(http::Error::NotFound {
            uri: context.request.uri().as_ref().to_string(),
        })
    }
}
