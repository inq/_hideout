use core::http::Response;

pub fn index() -> Response {
    Response::new_html(
        200,
        &tent::html!(
            html
                head
                    meta charset="utf-8"
                    link rel="stylesheet" href="/main.css"
                body
                    header id="mainHeader"
                        nav
                            a href="/session/new"
                                "Sign in"
                        p
                            "inkyu"
                            span.smRed
                                ".kr"
                    article
                        header
                            h1
                                "Lorem ipsum"
                        p
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    footer id="mainFooter"
                        "Inkyu © 2020 "
                        a href="https://github.com/inq/hideout"
                            svg version="1.1" height="16" width="16" viewBox="0 0 16 16"
                                path fillRule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
        )
        .to_string(),
    )
}

pub fn session_new() -> Response {
    Response::new_html(
        200,
        &tent::html!(
            html
                head
                    meta charset="utf-8"
                    link rel="stylesheet" href="/main.css"
                body
                    header id="mainHeader"
                        nav
                            a href="/session/new"
                                "Sign in"
                        p
                            "inkyu"
                            span.smRed
                                ".kr"
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
                    footer id="mainFooter"
                        "Inkyu © 2020 "
                        a href="https://github.com/inq/hideout"
                            svg version="1.1" height="16" width="16" viewBox="0 0 16 16"
                                path fillRule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
        )
        .to_string(),
    )
}

pub fn session_create() -> Response {
    Response::new_html(
        200,
        &tent::html!(
            html
                body
                    "Session created"
        )
        .to_string(),
    )
}

pub fn article_show(article_id: &str) -> Response {
    Response::new_html(
        200,
        &tent::html!(
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
        &tent::html!(
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

pub fn stylesheet() -> Response {
    Response::new_html(
        200,
        &tent::css!(
            @fontFace
                fontFamily: "Raleway"
                src: "url('/assets/raleway-light.woff') format('woff')"

            body
                margin: "1em auto"
                maxWidth: "900px"
                fontFamily: "Raleway"
            #mainHeader
                nav
                    textAlign: "right"
                p
                    width: "100%"
                    textAlign: "center"
                    margin: "0.2em 0"
                    fontSize: "3em"
                    color: "#556677"
                    .smRed
                        color: "#996677"
            #mainFooter
                textAlign: "right"
            .inner
                display: "block"
                maxWidth: "800px"
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
            article
                form
                    width: "50%"
                    ul
                        .input
                            display: "flex"
                            padding: "0.2em"
                            label
                                flex: "5 2"
                                textAlign: "right"
                                paddingRight: "2em"
                            input
                                flex: "5 3"
                        .submit
                            paddingLeft: "80%"
                            input
                                height: "2em"
                input
                    display: "block"
        )
        .to_string(),
    )
}
