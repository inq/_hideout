mod fixture;

use hideout::util::{config, Config, Logger};

fn parse_fixture() -> Result<fixture::Fixture, Error> {
    use std::fs::File;

    let reader = File::open("config/fixture.yaml").map_err(Error::FixtureFileOpen)?;
    let res = serde_yaml::from_reader(reader).map_err(Error::YamlParse)?;
    Ok(res)
}

#[derive(Debug)]
enum Error {
    FixtureFileOpen(std::io::Error),
    SetLogger(log::SetLoggerError),
    Config(config::Error),
    YamlParse(serde_yaml::Error),
    DbConnection(mongodb::error::Error),
    CollectionCreation(mongodb::error::Error),
    BsonSerialize(bson::ser::Error),
    Insertion(mongodb::error::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Log
    color_backtrace::install();
    log::set_logger(&Logger).map_err(Error::SetLogger)?;
    log::set_max_level(log::LevelFilter::Debug);

    // Config
    let config = Config::from_file("config/config.yaml").map_err(Error::Config)?;

    // Database
    let client = mongodb::Client::with_uri_str(config.db_uri())
        .await
        .map_err(Error::DbConnection)?;
    let db = client.database(config.db_name());

    let fixture = parse_fixture()?;

    let _res = db.collection("articles").drop(None).await;
    let _res = db
        .create_collection("articles", None)
        .await
        .map_err(Error::CollectionCreation)?;
    let _res = db.collection("users").drop(None).await;
    let _res = db
        .create_collection("users", None)
        .await
        .map_err(Error::CollectionCreation)?;
    let _res = db.collection("workouts").drop(None).await;
    let _res = db
        .create_collection("workouts", None)
        .await
        .map_err(Error::CollectionCreation)?;

    let _res = db.collection("dailies").drop(None).await;
    let _res = db
        .create_collection("dailies", None)
        .await
        .map_err(Error::CollectionCreation)?;

    for user_fixture in fixture.users.iter() {
        let password_hashed = hideout::util::Password::new(&user_fixture.password).hashed();
        let user = app::models::User::new(
            None,
            user_fixture.email.clone(),
            user_fixture.name.clone(),
            password_hashed,
        );
        if let bson::Bson::Document(document) =
            bson::to_bson(&user).map_err(Error::BsonSerialize)?
        {
            let _res = db
                .collection("users")
                .insert_one(document, None)
                .await
                .map_err(Error::Insertion)?;
        }
    }

    for workout_fixture in fixture.workouts.iter() {
        let workout = app::models::Workout::new(
            None,
            workout_fixture.name.clone(),
            workout_fixture.description.clone(),
            workout_fixture.with_barbell,
        );
        if let bson::Bson::Document(document) =
            bson::to_bson(&workout).map_err(Error::BsonSerialize)?
        {
            let _res = db
                .collection("workouts")
                .insert_one(document, None)
                .await
                .map_err(Error::Insertion)?;
        }
    }

    Ok(())
}
