mod cookie;
mod form_data;
mod request;
mod response;
mod status;
mod uri;

pub use cookie::Cookie;
pub use form_data::FormData;
pub use request::{Method, Request};
pub use response::Response;
pub use status::StatusCode;
pub use uri::Uri;

pub enum Error {
    NotFound { uri: String },
}

impl Error {
    pub fn status_code(&self) -> i32 {
        match self {
            Error::NotFound { .. } => 404,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
