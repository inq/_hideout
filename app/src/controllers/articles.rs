use crate::Context;
use hideout::http;

pub(super) struct Articles {}

impl Articles {
    pub(super) async fn serve_inner(
        context: Context,
        _payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "list" => Ok(Self::list(context)),
                article_id => Ok(Self::show(context, article_id)),
            }
        } else {
            Err(http::Error::NotFound {
                uri: context.request.uri().as_ref().to_string(),
            })
        }
    }

    fn show(context: Context, article_id: &str) -> http::Response {
        http::Response::new_html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article
                        span.label {format!("Show Article: {}", article_id)}
                )
                .to_string(),
            ),
        )
    }

    fn list(context: Context) -> http::Response {
        http::Response::new_html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article
                        span.label "List of articles"
                )
                .to_string(),
            ),
        )
    }
}
