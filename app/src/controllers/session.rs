use hideout::http::{Request, Response};
pub(super) struct Session {}

impl Session {
    pub(super) async fn serve_inner(request: Request, payload: &[u8], idx: usize) -> Response {
        match request.uri().nth_path(idx) {
            Some("new") => Self::handle_new(),
            Some("create") => Self::create(payload),
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

    fn create(payload: &[u8]) -> Response {
        use hideout::http::FormData;
        let form_data = FormData::parse_x_www_form_urlencoded(payload);

        Response::new_html(
            200,
            &super::render_with_layout(
                &tent::html!(
                    article
                        {format!("{:?}", form_data)}
                )
                .to_string(),
            ),
        )
    }
}
