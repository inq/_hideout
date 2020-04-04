use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    database: Database,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    host: String,
    user: String,
    password: String,
    dbname: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, failure::Error> {
        let contents = std::fs::read_to_string(path)?;
        let res = serde_yaml::from_str(&contents)?;
        Ok(res)
    }

    pub fn database_string(&self) -> String {
        // TODO: Implement host
        format!(
            "host={} user={} password='{}' dbname={}",
            self.database.host, self.database.user, self.database.password, self.database.dbname
        )
    }
}
