use crate::{models, Context};
use hideout::http;

pub(super) struct Session {}

#[derive(Debug)]
enum Error {
    Payload(hideout::http::form_data::Error),
    NoEmail,
    NoPassword,
    Credential,
    Query(mongodb::error::Error),
}

impl Session {
    pub(super) async fn serve_inner(
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "new" => Ok(Self::action_new(context)),
                "create" => Ok(Self::action_create(context, payload).await),
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

    fn action_new(context: Context) -> http::Response {
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article.signin
                        header
                            h1
                                "SIGN IN"
                        form action="/session/create" method="POST"
                            input type="text" name="email" placeholder="EMAIL"
                            input type="password" name="password" placeholder="PASSWORD"
                            input.submit type="submit" value="SUBMIT"
                )
                .to_string(),
            ),
        )
    }

    async fn action_create(mut context: Context, payload: &[u8]) -> http::Response {
        let inner: Result<http::Response, Error> = try {
            use hideout::http::FormData;

            let model = crate::models::Model::from_context(&context);

            let form_data =
                FormData::parse_x_www_form_urlencoded(payload).map_err(Error::Payload)?;
            let email = form_data.get("email").ok_or(Error::NoEmail)?;
            let password = form_data.get("password").ok_or(Error::NoPassword)?;

            let session = {
                let user = model
                    .users()
                    .auth(email, password)
                    .await
                    .map_err(Error::Query)?
                    .ok_or(Error::Credential)?;

                models::Session::new(user)
            };
            let key = context.server_state.add_session(session);

            http::Response::redirect_to(
                vec![format!("SID={}; Path=/", key.as_ref())],
                "/".to_string(),
            )
        };

        inner.unwrap_or_else(|e| {
            // TODO: Implement redirection & go back to login form
            http::Response::html(
                200,
                vec![],
                &super::render_with_layout(
                    &context,
                    &tent::html!(
                        article
                            {format!("{:?}", e)}
                    )
                    .to_string(),
                ),
            )
        })
    }
}
