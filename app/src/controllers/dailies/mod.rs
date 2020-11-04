use crate::Context;
use hideout::http;

pub(super) struct Dailies {}

#[derive(Debug)]
enum Error {
    Payload(hideout::http::form_data::Error),
    NoDate,
    InvalidDate(chrono::ParseError),
    Query(mongodb::error::Error),
}

impl Dailies {
    pub(super) async fn serve_inner(
        context: Context,
        payload: &[u8],
        idx: usize,
    ) -> http::Result<http::Response> {
        if let Some(path) = context.request.uri().nth_path(idx) {
            match path.as_ref() {
                "new" => Ok(Self::new(context)),
                "create" => Ok(Self::create(context, payload).await),
                article_id => Ok(Self::show(context, article_id)),
            }
        } else {
            Ok(Self::list(context))
        }
    }

    fn render_input_form(context: Context, flash: Option<&str>) -> http::Response {
        let flash_rendered = flash.map_or(String::from(""), |flash| {
            tent::html!(
                .flash
                    {flash}
            )
            .to_string()
        });
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article.signin
                        header
                            h1
                                "Daily"
                            {flash_rendered}
                        form action="/dailies/create" method="POST"
                            input type="date" name="date" placeholder="2020-08-23"
                            input.submit type="submit" value="SUBMIT"
                )
                .to_string(),
            ),
        )
    }

    fn new(context: Context) -> http::Response {
        Self::render_input_form(context, None)
    }

    async fn create(context: Context, payload: &[u8]) -> http::Response {
        let inner: Result<http::Response, Error> = try {
            use hideout::http::FormData;

            let model = crate::models::Model::from_context(&context);

            let form_data =
                FormData::parse_x_www_form_urlencoded(payload).map_err(Error::Payload)?;
            let date_str = form_data.get("date").ok_or(Error::NoDate)?;
            let naive_date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(Error::InvalidDate)?;
            let date = chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDateTime::new(naive_date, chrono::NaiveTime::from_hms(0, 0, 0)),
                chrono::Utc,
            );

            let _ = model
                .daily_activities()
                .create(date.with_timezone(&chrono::Utc))
                .await
                .map_err(Error::Query)?;

            http::Response::redirect_to(vec![], "/dailies/new".to_string())
        };

        inner.unwrap_or_else(|e| Self::render_input_form(context, Some(&format!("{:?}", e))))
    }

    fn show(context: Context, article_id: &str) -> http::Response {
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article
                        span.label {format!("Show Article: {}", article_id)}
                )
                .to_string(),
            ),
        )
    }

    fn list(context: Context) -> http::Response {
        http::Response::html(
            200,
            vec![],
            &super::render_with_layout(
                &context,
                &tent::html!(
                    article
                        div.header
                            input type="date"
                        span.label "List of articles"
                )
                .to_string(),
            ),
        )
    }
}
