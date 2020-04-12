use hideout::{
    http::{Request, Response},
    model::Context,
};
pub(super) struct Session {}

impl Session {
    pub(super) async fn serve_inner(
        request: Request,
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> Response {
        match request.uri().nth_path(idx) {
            Some("new") => Self::handle_new(),
            Some("create") => Self::create(context, payload).await,
            _ => crate::handlers::not_found(request.uri().as_str()),
        }
    }

    fn handle_new() -> Response {
        Response::new_html(
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

    async fn create(context: Context, payload: &[u8]) -> Response {
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

            Response::new_html(
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
            Response::new_html(
                200,
                &super::render_with_layout(
                    &tent::html!({ format!("NOT FOUND: {}", email) }).to_string(),
                ),
            )
        }
    }
}
