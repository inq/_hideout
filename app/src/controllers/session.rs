use hideout::{http, model::Context};

pub(super) struct Session {}

impl Session {
    pub(super) async fn serve_inner(
        request: http::Request,
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        match request.uri().nth_path(idx) {
            Some("new") => Ok(Self::handle_new()),
            Some("create") => Ok(Self::create(context, payload).await),
            _ => Err(http::Error::NotFound {
                uri: request.uri().as_str().to_string(),
            }),
        }
    }

    fn handle_new() -> http::Response {
        http::Response::new_html(
            200,
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

    async fn create(context: Context, payload: &[u8]) -> http::Response {
        // TODO: Handle errors
        use hideout::http::FormData;
        use hideout::util::Password;

        let form_data = FormData::parse_x_www_form_urlencoded(payload).unwrap();

        let email = form_data.get("email").unwrap();
        let password_hashed = Password::new(&form_data.get("password").unwrap()).hashed();

        let rows = context
            .db
            .query(
                "SELECT * FROM users WHERE email=$1 AND password_hashed=$2",
                &[email, &password_hashed],
            )
            .await
            .unwrap();

        if rows.len() == 1 {
            let row = &rows[0];

            http::Response::new_html(
                200,
                &super::render_with_layout(
                    &tent::html!(
                        article
                            {format!("{:?}", row)}
                    )
                    .to_string(),
                ),
            )
        } else {
            http::Response::new_html(
                200,
                &super::render_with_layout(
                    &tent::html!({ format!("NOT FOUND: {}", email) }).to_string(),
                ),
            )
        }
    }
}
