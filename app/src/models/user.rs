use bson::doc;
use serde::{Deserialize, Serialize};

pub struct Users {
    pub(super) db: mongodb::Database,
}

impl Users {
    pub async fn auth(
        self,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, mongodb::error::Error> {
        use hideout::util::Password;

        let password_hashed = Password::new(password).hashed();

        let filter = doc! {"email": email, "password_hashed": password_hashed };
        if let Some(doc) = self.db.collection("users").find_one(filter, None).await? {
            let user = bson::from_bson(bson::Bson::Document(doc))?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    email: String,
    name: String,
    password_hashed: String,
}

impl User {
    pub fn new(
        id: Option<bson::oid::ObjectId>,
        email: String,
        name: String,
        password_hashed: String,
    ) -> Self {
        Self {
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
