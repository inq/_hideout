use crate::Context;
use hideout::http;

pub(super) struct Workouts {}

impl Workouts {
    pub(super) async fn serve_inner(
        context: Context,
        _payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if context.session.is_none() {
            return Err(http::Error::Unauthorized {
                uri: context.request.uri().as_ref().to_string(),
            });
        }

        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "list" => Ok(Self::list(context).await),
                _ => Err(http::Error::NotFound {
                    uri: context.request.uri().as_ref().to_string(),
                }),
            }
        } else {
            Err(http::Error::NotFound {
                uri: context.request.uri().as_ref().to_string(),
            })
        }
    }

    async fn list(context: Context) -> http::Response {
        let model = crate::models::Model::from_context(&context);
        // TODO: Remove unwrap
        let workouts = model.workouts().all().await.unwrap();

        http::Response::html(
            200,
            vec![],
            &crate::controllers::render_with_layout(
                &context,
                &tent::html!(
                    workouts
                        span.label
                            {format!("{:?}", workouts)}
                )
                .to_string(),
            ),
        )
    }
}
