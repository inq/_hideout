use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: Uuid,
    email: String, // TODO: Use Bytes
    name: String,
    password_hashed: String,
}

impl User {
    pub fn new(id: Uuid, email: String, name: String, password_hashed: String) -> User {
        User {
            id,
            email,
            name,
            password_hashed,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}
