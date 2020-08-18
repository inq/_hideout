mod daily_activity;
mod session;
mod user;
mod workout;

pub use daily_activity::{DailyActivities, DailyActivity};
pub use session::Session;
pub use user::{User, Users};
pub use workout::{Workout, Workouts};

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

    pub fn workouts(self) -> Workouts {
        Workouts { db: self.db }
    }
}
