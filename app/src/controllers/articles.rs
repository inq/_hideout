use hideout::http::{Request, Response};
use hideout::model::Context;
pub(super) struct Articles {}

impl Articles {
    pub(super) async fn serve_inner(
        request: Request,
        context: Context,
        _payload: &[u8],
        idx: usize,
    ) -> Response {
        match request.uri().nth_path(idx) {
            Some("list") => Self::list(context),
            Some(article_id) => Self::article_show(context, article_id),
            _ => crate::handlers::not_found(request.uri().as_str()),
        }
    }

    fn article_show(_context: Context, article_id: &str) -> Response {
        Response::new_html(
            200,
            &super::render_with_layout(
                &tent::html!(
                    article
                        span.label {format!("Show Article: {}", article_id)}
                )
                .to_string(),
            ),
        )
    }

    fn list(_context: Context) -> Response {
        Response::new_html(
            200,
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
