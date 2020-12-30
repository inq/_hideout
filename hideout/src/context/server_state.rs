use super::session_store::{self, SessionStore};
use std::rc::Rc;

pub struct ServerState<S> {
    pub db: mongodb::Database,
    rng: rand::rngs::ThreadRng,
    sessions: SessionStore<S>,
}

impl<S> std::clone::Clone for ServerState<S> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            sessions: self.sessions.clone(),
            rng: self.rng.clone(),
        }
    }
}

impl<S> ServerState<S> {
    pub async fn new(config: crate::util::Config) -> Result<Self, mongodb::error::Error> {
        let client = mongodb::Client::with_uri_str(config.db_uri()).await?;
        let db = client.database(config.db_name());

        Ok(Self {
            db,
            sessions: SessionStore::<S>::new(),
            rng: rand::thread_rng(),
        })
    }

    pub fn add_session(&mut self, session: S) -> session_store::Key {
        let key = session_store::Key::new(crate::util::Uuid::new_v4(&mut self.rng).to_string());
        self.sessions.set(&key, session);
        key
    }

    pub fn get_session(&self, key: &str) -> Option<Rc<S>> {
        self.sessions.get(&session_store::Key::new(key.to_string()))
    }
}
