pub mod session_store;
use session_store::SessionStore;
use std::rc::Rc;

pub struct Context<T> {
    pub db: Rc<tokio_postgres::Client>,
    pub session: SessionStore<T>,
    pub rng: rand::rngs::ThreadRng,
}

impl<T> std::clone::Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            session: self.session.clone(),
            rng: self.rng,
        }
    }
}

impl<T> Context<T> {
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
            session: SessionStore::<T>::new(),
            rng: rand::thread_rng(),
        })
    }
}
