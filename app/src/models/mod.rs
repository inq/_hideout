mod session;
mod user;

pub use session::Session;
pub use user::{User, Users};

pub struct Model {
    db: mongodb::Database,
}

impl Model {
    pub fn from_context(context: &crate::Context) -> Self {
        Self {
            db: context.server_state.db.clone(),
        }
    }

    pub fn users(self) -> Users {
        Users { db: self.db }
    }
}
