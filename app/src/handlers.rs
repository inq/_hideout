use core::http::Response;

pub fn index() -> Response {
    Response::new_html(
        200,
        tent::html!(
            html
                body
                    span.label "Index page"
        )
        .to_string(),
    )
}

pub fn article_show(article_id: &str) -> Response {
    Response::new_html(
        200,
        tent::html!(
            html
                body
                    span.label {format!("Show Article: {}", article_id)}
        )
        .to_string(),
    )
}

pub fn article_list() -> Response {
    Response::new_html(
        200,
        tent::html!(
            html
                body
                    span.label "List of articles"
        )
        .to_string(),
    )
}

// TODO: Move this into separated module
pub fn not_found(uri: &str) -> Response {
    Response::new_html(
        404,
        tent::html!(
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

pub fn stylesheet() -> Response {
    Response::new_html(
        200,
        tent::css!(
            body
                fontFamily: "sans-serif"
            .notice
                width: "400px"
                height: "300px"
                margin: "auto"
                fontSize: "2em"
                textAlign: "center"
                .head
                    fontSize: "4em"
                .content
                    fontSize: "0.5em"
                    lineHeight: "1.5em"
        )
        .to_string(),
    )
}
