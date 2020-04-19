use super::session_store::{self, SessionStore};
use std::rc::Rc;

pub struct ServerState<S> {
    pub db: Rc<tokio_postgres::Client>,
    rng: rand::rngs::ThreadRng,
    sessions: SessionStore<S>,
}

impl<S> std::clone::Clone for ServerState<S> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            sessions: self.sessions.clone(),
            rng: self.rng,
        }
    }
}

impl<S> ServerState<S> {
    pub async fn new(config: crate::util::Config) -> Result<Self, failure::Error> {
        let (client, connection) =
            tokio_postgres::connect(&config.database_string(), tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self {
            db: Rc::new(client),
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
