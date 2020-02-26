use serde::Deserialize;
use std::fs::File;

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
    pub fn from_file(path: &str) -> Result<Self, failure::Error> {
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

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
