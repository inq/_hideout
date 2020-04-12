use std::rc::Rc;

#[derive(Clone)]
pub struct Context {
    pub db: Rc<tokio_postgres::Client>,
}

impl Context {
    pub async fn new(config: crate::util::Config) -> Result<Self, failure::Error> {
        let (client, connection) =
            tokio_postgres::connect(&config.database_string(), tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self {
            db: Rc::new(client),
        })
    }
}
