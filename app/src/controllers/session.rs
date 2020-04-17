use crate::{models, Context};
use failure::Fail;
use hideout::http;

pub(super) struct Session {}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "invalid payload")]
    InvalidPayload,
    #[fail(display = "invalid credential")]
    InvalidCredential,
}

impl Session {
    pub(super) async fn serve_inner(
        request: http::Request,
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        match request.uri().nth_path(idx) {
            Some("new") => Ok(Self::session_new()),
            Some("create") => Ok(Self::create(context, payload).await),
            _ => Err(http::Error::NotFound {
                uri: request.uri().as_ref().to_string(),
            }),
        }
    }

    fn session_new() -> http::Response {
        http::Response::new_html(
            200,
            vec![],
            &super::render_with_layout(
                &tent::html!(
                    article
                        header
                            h1
                                "Signin"
                        form action="/session/create" method="POST"
                            ul
                                li.input
                                    label for="email" "email"
                                    input type="text" name="email"
                                li.input
                                    label for="password" "password"
                                    input type="password" name="password"
                                li.submit
                                    input type="submit"
                )
                .to_string(),
            ),
        )
    }

    async fn create(mut context: Context, payload: &[u8]) -> http::Response {
        let inner: Result<http::Response, Error> = try {
            use hideout::http::FormData;

            let model = crate::models::Model::from_context(context.clone());

            let form_data = FormData::parse_x_www_form_urlencoded(payload)
                .map_err(|_| Error::InvalidPayload)?;
            let email = form_data.get("email").ok_or(Error::InvalidPayload)?;
            let password = form_data.get("password").ok_or(Error::InvalidPayload)?;

            let session = {
                let user = model
                    .users()
                    .auth(email, password)
                    .await
                    .ok_or(Error::InvalidCredential)?;

                models::Session::new(user)
            };
            let key = context.add_session(session);

            http::Response::new_html(
                200,
                vec![],
                &super::render_with_layout(
                    &tent::html!(
                        article
                            {format!("Hello, {:?}", key)}
                    )
                    .to_string(),
                ),
            )
        };

        inner.unwrap_or_else(|e| {
            // TODO: Implement redirection & go back to login form
            http::Response::new_html(
                200,
                vec![String::from("SID=1234")],
                &super::render_with_layout(
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
