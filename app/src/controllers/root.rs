use crate::Context;
use hideout::http;

pub struct Root {}

impl Root {
    pub(super) async fn serve_inner(
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "articles" => super::Articles::serve_inner(context, payload, idx + 1).await,
                "assets" => super::Assets::serve_inner(context, payload, idx + 1).await,
                "session" => super::Session::serve_inner(context, payload, idx + 1).await,
                "main.css" => Ok(Self::stylesheet()),
                _ => Err(http::Error::NotFound {
                    uri: context.request.uri().as_ref().to_string(),
                }),
            }
        } else {
            Ok(Self::index(context))
        }
    }

    fn index(context: Context) -> http::Response {
        let content = r#"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
            incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud
            exercitation ullamco laboris nisi ut aliquip ex ea commodo consequa. Duis aute
            irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
            pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia
            deserunt mollit anim id est laborum."#;
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article
                        header
                            h1
                                "Lorem ipsum"
                            p
                                {content}
                )
                .to_string(),
            ),
        )
    }

    fn stylesheet() -> http::Response {
        http::Response::html(
            200,
            vec![],
            &tent::css!(
                @fontFace
                    fontFamily: "Raleway"
                    src: "url('/assets/raleway-light.woff') format('woff')"

                body
                    margin: "1em auto"
                    maxWidth: "900px"
                    fontFamily: "Raleway"
                a
                    color: "inherit"
                    textDecoration: "none"
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

    pub async fn serve(context: Context, payload: &[u8]) -> http::Result<http::Response> {
        Self::serve_inner(context, payload, 0).await
    }
}
