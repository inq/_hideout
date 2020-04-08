use hideout::http::{Request, Response};
pub(super) struct Articles {}

impl Articles {
    pub(super) async fn serve_inner(request: Request, _payload: &[u8], idx: usize) -> Response {
        match request.uri().nth_path(idx) {
            Some("list") => Self::list(),
            Some(article_id) => Self::article_show(article_id),
            _ => crate::handlers::not_found(request.uri().as_str()),
        }
    }

    fn article_show(article_id: &str) -> Response {
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

    fn list() -> Response {
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
