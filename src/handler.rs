use crate::http::Response;

pub fn index(uri: &str) -> Response {
    let payload = format!(
        "{}",
        tent::html!(
            html
                span.hello "Index page"
                .hello {format!("given uri: {}", uri)}
        )
    );
    Response::new_html(200, payload)
}

pub fn hello(uri: &str) -> Response {
    let payload = format!(
        "{}",
        tent::html!(
            html
                span.hello "HELLO!"
                .hello {format!("given uri: {}", uri)}
        )
    );
    Response::new_html(200, payload)
}

pub fn hello_world(uri: &str) -> Response {
    let payload = format!(
        "{}",
        tent::html!(
            html
                span.hello "HELLO, WORLD!"
                .hello {format!("given uri: {}", uri)}
        )
    );
    Response::new_html(200, payload)
}

// TODO: Move this into separated module
pub fn not_found(uri: &str) -> Response {
    let payload = format!(
        "{}",
        tent::html!(
            html
                span.hello "404 Not Found"
                .hello {format!("given uri: {}", uri)}
        )
    );
    Response::new_html(404, payload)
}

pub fn stylesheet(_uri: &str) -> Response {
    let payload = r#"
    body {
        text-align: center;
    }"#;
    Response::new_html(200, payload.to_string())
}
