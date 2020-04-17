mod session;
mod user;

pub use session::Session;
pub use user::{User, Users};

pub struct Model {
    context: crate::Context,
}

impl Model {
    pub fn from_context(context: crate::Context) -> Self {
        Self { context }
    }

    pub fn users(self) -> Users {
        Users {
            context: self.context,
        }
    }
}
