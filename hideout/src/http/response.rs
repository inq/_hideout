use crate::http::status::StatusCode;
use std::fmt;

pub struct Response {
    pub header: Header,
    pub payload: Vec<u8>,
}

pub struct Header {
    version: String,
    status: StatusCode,
    content_type: String,
    connection: String,
    content_length: usize,
}

impl Response {
    pub fn new_html(code: u16, payload_str: &str) -> Self {
        let payload = payload_str.as_bytes().to_vec();
        Self {
            header: Header {
                version: String::from("1.1"),
                status: StatusCode(code),
                content_type: String::from("text/html"),
                connection: String::from("Closed"),
                content_length: payload.len(),
            },
            payload,
        }
    }

    pub fn new_binary(code: u16, payload: &[u8], content_type: &str) -> Self {
        Self {
            header: Header {
                version: String::from("1.1"),
                status: StatusCode(code),
                content_type: content_type.to_string(),
                connection: String::from("Closed"),
                content_length: payload.len(),
            },
            payload: payload.to_vec(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP/{} {}\r\n", self.version, self.status)?;
        write!(f, "Content-Type: {}\r\n", self.content_type)?;
        write!(f, "Content-Length: {}\r\n", self.content_length)?;
        write!(f, "Connection: {}\r\n", self.connection)?;
        write!(f, "\r\n")
    }
}
