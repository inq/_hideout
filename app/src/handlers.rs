use hideout::http::Response;

pub fn not_found(uri: &str) -> Response {
    Response::html(
        404,
        vec![],
        &tent::html!(
            html
                head
                    link rel="stylesheet" href="/main.css"
                body
                    .notice
                        .head "404"
                        .content "NOT FOUND"
                        .detail {format!("given uri: {}", uri)}
        )
        .to_string(),
    )
}
