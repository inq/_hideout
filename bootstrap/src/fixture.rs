use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fixture {
    pub users: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
}
