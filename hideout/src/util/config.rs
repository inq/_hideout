use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    database: Database,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    uri: String,
    db_name: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, failure::Error> {
        let contents = std::fs::read_to_string(path)?;
        let res = serde_yaml::from_str(&contents)?;
        Ok(res)
    }

    pub fn db_uri(&self) -> &str {
        &self.database.uri
    }

    pub fn db_name(&self) -> &str {
        &self.database.db_name
    }
}
