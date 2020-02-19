use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Config {
    database: Database,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    username: String,
    password: String,
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
}
