use crate::Context;
use hideout::http;

pub(super) struct Private {}

impl Private {
    pub(super) async fn serve_inner(
        context: Context,
        _payload: &[u8],
        _idx: usize,
    ) -> http::Result<http::Response> {
        if context.session.is_none() {
            return Err(http::Error::Unauthorized {
                uri: context.request.uri().as_ref().to_string(),
            });
        }

        Err(http::Error::NotFound {
            uri: context.request.uri().as_ref().to_string(),
        })
    }
}
