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
                "private" => super::private::Private::serve_inner(context, payload, idx + 1).await,
                "articles" => super::Articles::serve_inner(context, payload, idx + 1).await,
                "assets" => super::Assets::serve_inner(context, payload, idx + 1).await,
                "session" => super::Session::serve_inner(context, payload, idx + 1).await,
                "main.css" => Ok(Self::stylesheet()),
                _ => Err(http::Error::NotFound {
                    uri: context.request.uri().as_ref().to_string(),
                }),
            }
        } else {
            Ok(super::Articles::action_list(context))
        }
    }

    fn stylesheet() -> http::Response {
        // TODO: Process article.signin correctly
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
                .signin
                    h1
                        color: "#666666"
                        textAlign: "center"
                    margin: "20px auto"
                    padding: "10px"
                    width: "50%"
                    border: "1px solid #ddd"
                    borderRadius: "0.2em"
                article
                    input
                        display: "block"
                        width: "80%"
                        height: "3em"
                        lineHeight: "2em"
                        margin: "1em auto"
                        boxSizing: "border-box"
                        padding: "0 2em"
                        border: "1px solid #dddddd"
                        borderLeft: "0px"
                        borderTop: "0px"
                        borderRight: "0px"
                    .submit
                        color: "#fff"
                        width: "40%"
                        margin: "2em 10% 2em auto"
                        borderRadius: "0.3em"
            )
            .to_string(),
        )
    }

    pub async fn serve(context: Context, payload: &[u8]) -> http::Result<http::Response> {
        Self::serve_inner(context, payload, 0).await
    }
}
