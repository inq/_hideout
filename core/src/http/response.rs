use crate::http::status::StatusCode;
use std::fmt;

pub struct Response {
    version: String,
    status: StatusCode,
    // Header
    content_type: String,
    connection: String,
    // Payload
    payload: String,
}

impl Response {
    pub fn new_html(code: u16, payload: String) -> Self {
        Self {
            version: String::from("1.1"),
            status: StatusCode(code),
            content_type: String::from("text/html"),
            connection: String::from("Closed"),
            payload,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP/{} {}\r\n", self.version, self.status)?;
        write!(f, "Content-Type: {}\r\n", self.content_type)?;
        write!(f, "Content-Length: {}\r\n", self.payload.as_bytes().len())?;
        write!(f, "Connection: {}\r\n", self.connection)?;
        write!(f, "\r\n{}", self.payload)
    }
}
