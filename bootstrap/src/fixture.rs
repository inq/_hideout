use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fixture {
    pub users: Vec<User>,
    pub workouts: Vec<Workout>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Workout {
    pub name: String,
    pub description: String,
    #[serde(rename = "withBarbell")]
    pub with_barbell: bool,
}
