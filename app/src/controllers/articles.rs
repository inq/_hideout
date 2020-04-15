use crate::Context;
use hideout::http;

pub(super) struct Articles {}

impl Articles {
    pub(super) async fn serve_inner(
        request: http::Request,
        context: Context,
        _payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        match request.uri().nth_path(idx) {
            Some("list") => Ok(Self::list(context)),
            Some(article_id) => Ok(Self::show(context, article_id)),
            _ => Err(http::Error::NotFound {
                uri: request.uri().as_ref().to_string(),
            }),
        }
    }

    fn show(_context: Context, article_id: &str) -> http::Response {
        http::Response::new_html(
            200,
            vec![],
            &super::render_with_layout(
                &tent::html!(
                    article
                        span.label {format!("Show Article: {}", article_id)}
                )
                .to_string(),
            ),
        )
    }

    fn list(_context: Context) -> http::Response {
        http::Response::new_html(
            200,
            vec![],
            &super::render_with_layout(
                &tent::html!(
                    article
                        span.label "List of articles"
                )
                .to_string(),
            ),
        )
    }
}
