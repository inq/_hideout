use crate::Context;
use hideout::http;

pub(super) struct Dailies {}

impl Dailies {
    pub(super) async fn serve_inner(
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "new" => Ok(Self::new(context)),
                "create" => Ok(Self::create(context, payload)),
                article_id => Ok(Self::show(context, article_id)),
            }
        } else {
            Ok(Self::list(context))
        }
    }

    fn new(context: Context) -> http::Response {
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article.signin
                        header
                            h1
                                "Daily"
                        form action="/dailies/create" method="POST"
                            input type="date" name="date" placeholder="2020-08-23"
                            input.submit type="submit" value="SUBMIT"
                )
                .to_string(),
            ),
        )
    }

    fn create(context: Context, payload: &[u8]) -> http::Response {
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article.signin
                        header
                            h1
                                "Daily"
                        form action="/dailies/create" method="POST"
                            input type="date" name="date" placeholder="2020-08-23"
                            input.submit type="submit" value="SUBMIT"
                )
                .to_string(),
            ),
        )
    }

    fn show(context: Context, article_id: &str) -> http::Response {
        http::Response::html(
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
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article
                        div.header
                            input type="date"
                        span.label "List of articles"
                )
                .to_string(),
            ),
        )
    }
}
