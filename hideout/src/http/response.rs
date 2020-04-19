use crate::http::status::StatusCode;
use std::fmt;

pub struct Response {
    pub header: Header,
    pub payload: Vec<u8>,
}

pub struct Header {
    version: String,
    status: StatusCode,
    content_type: Option<String>,
    connection: Option<String>,
    content_length: Option<usize>,
    location: Option<String>,
    set_cookies: Vec<String>, // TODO: Make a struct to represent this
}

impl Response {
    pub fn html(code: u16, set_cookies: Vec<String>, payload_str: &str) -> Self {
        let payload = payload_str.as_bytes().to_vec();
        Self {
            header: Header {
                version: String::from("1.1"),
                status: StatusCode(code),
                content_type: Some(String::from("text/html")),
                connection: Some(String::from("Closed")),
                content_length: Some(payload.len()),
                location: None,
                set_cookies,
            },
            payload,
        }
    }

    pub fn binary(code: u16, set_cookies: Vec<String>, payload: &[u8], content_type: &str) -> Self {
        Self {
            header: Header {
                version: String::from("1.1"),
                status: StatusCode(code),
                content_type: Some(content_type.to_string()),
                connection: Some(String::from("Closed")),
                content_length: Some(payload.len()),
                location: None,
                set_cookies,
            },
            payload: payload.to_vec(),
        }
    }

    pub fn redirect_to(set_cookies: Vec<String>, location: String) -> Self {
        Self {
            header: Header {
                version: String::from("1.1"),
                status: StatusCode(301),
                content_type: None,
                connection: None,
                content_length: None,
                location: Some(location),
                set_cookies,
            },
            payload: vec![],
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP/{} {}\r\n", self.version, self.status)?;
        if let Some(content_type) = self.content_type.as_ref() {
            write!(f, "Content-Type: {}\r\n", content_type)?;
        }
        if let Some(content_length) = self.content_length {
            write!(f, "Content-Length: {}\r\n", content_length)?;
        }
        if let Some(connection) = self.connection.as_ref() {
            write!(f, "Connection: {}\r\n", connection)?;
        }
        if let Some(location) = self.location.as_ref() {
            write!(f, "Location: {}\r\n", location)?;
        }
        for set_cookie in self.set_cookies.iter() {
            write!(f, "Set-Cookie: {}\r\n", set_cookie)?;
        }
        write!(f, "\r\n")
    }
}
