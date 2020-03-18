mod fixture;

use core::{Config, Logger};
use std::fmt;

fn parse_fixture() -> Result<fixture::Fixture, failure::Error> {
    use std::fs::File;

    let reader = File::open("config/fixture.yaml")?;
    let res = serde_yaml::from_reader(reader)?;
    Ok(res)
}

struct HexBytes<'a>(&'a [u8]);

impl<'a> fmt::Display for HexBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // Log
    color_backtrace::install();
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    // Config
    let config = Config::from_file("config/config.yaml")?;

    // Database
    let (client, connection) =
        tokio_postgres::connect(&config.database_string(), tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let fixture = parse_fixture()?;

    let _res = client.query("DROP TABLE IF EXISTS articles", &[]).await?;

    let _res = client
        .query(
            r#"
        CREATE TABLE articles (
            id UUID DEFAULT uuid_generate_v4(),
            content TEXT NOT NULL,
            PRIMARY KEY (id)
        )"#,
            &[],
        )
        .await?;

    let _res = client.query("DROP TABLE IF EXISTS users", &[]).await?;

    let _res = client
        .query(
            r#"
        CREATE TABLE users (
            id UUID DEFAULT uuid_generate_v4(),
            email VARCHAR(255) NOT NULL UNIQUE,
            name VARCHAR(255) NOT NULL,
            password_hashed VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        )"#,
            &[],
        )
        .await?;

    for user in fixture.users.iter() {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.input(&user.password);
        let result = hasher.result();
        let password_hashed = HexBytes(&result).to_string();

        let _res = client
            .query(
                r#"
            INSERT INTO users (email, name, password_hashed)
            VALUES ($1::VARCHAR, $2::VARCHAR, $3::VARCHAR)
            "#,
                &[&user.email, &user.name, &password_hashed],
            )
            .await?;
    }

    Ok(())
}
