use core::{Config, Logger};

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // Log
    color_backtrace::install();
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    // Config
    let config = Config::from_file(".config.yaml")?;

    // Database
    let (client, connection) =
        tokio_postgres::connect(&config.database_string(), tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let res = client
        .query(
            r#"
        CREATE TABLE articles (
            id INTEGER PRIMARY KEY,
            content TEXT
        )"#,
            &[],
        )
        .await;
    if let Err(err) = res {
        panic!("Table creation erorr: {}", err.code().unwrap().code());
    }
    Ok(())
}
