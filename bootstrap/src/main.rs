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
    let client = mongodb::Client::with_uri_str(config.db_uri()).await?;
    let db = client.database(config.db_name());

    let fixture = parse_fixture()?;

    let _res = db.collection("articles").drop(None).await;
    let _res = db.create_collection("articles", None).await?;
    let _res = db.collection("users").drop(None).await;
    let _res = db.create_collection("users", None).await?;

    for user_fixture in fixture.users.iter() {
        let password_hashed = hideout::util::Password::new(&user_fixture.password).hashed();
        let user = app::models::User::new(
            None,
            user_fixture.email.clone(),
            user_fixture.name.clone(),
            password_hashed,
        );
        if let bson::Bson::Document(document) = bson::to_bson(&user)? {
            let _res = db.collection("users").insert_one(document, None).await?;
        }
    }

    Ok(())
}
