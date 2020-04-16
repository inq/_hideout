#[macro_use]
extern crate failure;
mod fixture;

use hideout::util::{Config, Logger};

fn parse_fixture() -> Result<fixture::Fixture, failure::Error> {
    use std::fs::File;

    let reader = File::open("config/fixture.yaml")?;
    let res = serde_yaml::from_reader(reader)?;
    Ok(res)
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "set_logger error")]
    SetLogger,
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // Log
    color_backtrace::install();
    log::set_logger(&Logger).map_err(|_| Error::SetLogger)?;
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
        let password_hashed = hideout::util::Password::new(&user.password).hashed();

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
