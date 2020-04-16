#![feature(try_blocks)]
use app::models::User;
use hideout::util::Config;
use std::env;
use std::path::PathBuf;
use tokio;

#[tokio::test]
async fn test_simple() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../config/config.yaml");

    let res: Result<_, failure::Error> = try {
        let config = Config::from_file(path)?;

        let (client, connection) =
            tokio_postgres::connect(&config.database_string(), tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client.query("SELECT * FROM users", &[]).await?;
        let _users = rows
            .iter()
            .map(|row| User::new(row.get(0), row.get(1), row.get(2), row.get(3)));
    };
    assert!(res.is_ok());
}
