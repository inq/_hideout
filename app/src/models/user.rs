use uuid::Uuid;

pub struct Users {
    pub(super) context: crate::Context,
}

impl Users {
    pub async fn auth(self, email: &str, password: &str) -> Option<User> {
        use hideout::util::Password;

        let password_hashed = Password::new(password).hashed();

        let rows = self
            .context
            .db
            .query(
                "SELECT * FROM users WHERE email=$1 AND password_hashed=$2",
                &[&email, &password_hashed],
            )
            .await
            .unwrap();

        if rows.len() != 1 {
            return None;
        }
        let row = &rows[0];
        Some(User {
            id: row.get(0),
            email: row.get(1),
            name: row.get(2),
            password_hashed: row.get(3),
        })
    }
}

#[derive(Debug, Clone)]
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
