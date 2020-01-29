use crate::http::Response;

pub fn index(uri: &str) -> Response {
    let payload = format!(
        "{}",
        tent::html!(
            html
                body
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
                body
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
                body
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
                head
                    link rel="stylesheet" href="/main.css"
                body
                    .notice
                        .head "404"
                        .content "NOT FOUND"
                        .detail {format!("given uri: {}", uri)}
        )
    );
    Response::new_html(404, payload)
}

pub fn stylesheet(_uri: &str) -> Response {
    let payload = r#"
    body {
        font-family: sans-serif;
    }
    .notice {
        width: 400px;
        height: 300px;
        margin: auto;
        font-size: 2em;
        text-align: center;
    }
    .notice .head {
        font-size: 4em;

    }
    .notice .content {
        font-size: 1em;
        line-height: 1.5em;
    }
    .notice .detail {
        font-size: 0.5em;
        line-height: 1.5em;
    }
    "#;
    Response::new_html(200, payload.to_string())
}
