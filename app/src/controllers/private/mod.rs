mod workouts;

use crate::Context;
use hideout::http;

pub(super) struct Private {}

impl Private {
    pub(super) async fn serve_inner(
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if context.session.is_none() {
            return Err(http::Error::Unauthorized {
                uri: context.request.uri().as_ref().to_string(),
            });
        }

        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "workouts" => workouts::Workouts::serve_inner(context, payload, idx + 1).await,
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
}
