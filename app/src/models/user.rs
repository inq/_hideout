use uuid::Uuid;

#[derive(Debug)]
pub struct User<'a> {
    id: Uuid,
    email: &'a str,
    name: &'a str,
    password_hashed: &'a str,
}

impl<'a> User<'a> {
    pub fn new<'b>(id: Uuid, email: &'b str, name: &'b str, password_hashed: &'b str) -> User<'b> {
        User {
            id,
            email,
            name,
            password_hashed,
        }
    }
}
