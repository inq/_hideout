use crate::http::Response;

pub fn index() -> Response {
    Response::new_html(
        200,
        tent::html!(
            html
                body
                    span.hello "Index page"
        )
        .to_string(),
    )
}

pub fn hello() -> Response {
    Response::new_html(
        200,
        tent::html!(
            html
                body
                    span.hello "/hello"
        )
        .to_string(),
    )
}

pub fn hello_world() -> Response {
    Response::new_html(
        200,
        tent::html!(
            html
                body
                    span.hello "HELLO, WORLD!"
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
