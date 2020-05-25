#![feature(try_blocks)]
use app::models::User;
use futures::stream::StreamExt;
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
        let client = mongodb::Client::with_uri_str(config.db_uri()).await?;
        let db = client.database(config.db_name());

        let rows = db.collection("users").find(None, None).await?;
        let users = rows
            .map(|row| bson::from_bson::<User>(bson::Bson::Document(row.ok()?)).ok())
            .collect::<Vec<_>>()
            .await;
        assert_eq!(users.len(), 1);
    };
    assert!(res.is_ok());
}
