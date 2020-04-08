use crate::http::Uri;
use bytes::Bytes;
use std::fmt::{self, Debug};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid value: {}", value)]
    ValueError { value: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
}

impl std::str::FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "OPTIONS" => Method::Options,
            "HEAD" => Method::Head,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "TRACE" => Method::Trace,
            "CONNECT" => Method::Connect,
            etc => {
                return Err(Error::ValueError {
                    value: etc.to_string(),
                })
            }
        })
    }
}

#[derive(Clone, Debug)]
pub enum Version {
    Http10,
    Http11,
}

impl std::convert::From<u8> for Version {
    fn from(value: u8) -> Self {
        if value == 0 {
            Version::Http10
        } else {
            Version::Http11
        }
    }
}

pub struct RequestLine {
    method: Method,
    uri: Uri,
    version: Version,
}

impl RequestLine {
    pub fn from_parsed(buffer: &Bytes, parsed: &httparse::Request) -> Result<Self, failure::Error> {
        use std::str::FromStr;

        let method = Method::from_str(parsed.method.unwrap())?;
        let uri = Uri::from_bytes(&slice_to_bytes(buffer, parsed.path.unwrap().as_bytes()))?;
        let version = parsed.version.unwrap().into();

        Ok(Self {
            method,
            uri,
            version,
        })
    }
}

impl Debug for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RequestLine {{ {:?} {:?} {:?} }}",
            self.method, self.uri, self.version
        )
    }
}

fn slice_to_bytes(buffer: &Bytes, slice: &[u8]) -> Bytes {
    let boundary = unsafe {
        buffer
            .as_ptr()
            .add(buffer.len())
            .offset_from(slice.as_ptr().add(slice.len()))
    };
    let offset = unsafe { slice.as_ptr().offset_from(buffer.as_ptr()) };
    assert!(offset >= 0 && boundary >= 0, "{}, {}", offset, boundary);
    let offset = offset as usize;
    buffer.slice(offset..offset + slice.len())
}

#[derive(Debug)]
pub struct Request {
    request_line: RequestLine,
    headers: Vec<Header>,
    pub body: Bytes,
}

pub struct Header {
    name: Bytes,
    value: Bytes,
}

impl Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?}",
            std::str::from_utf8(&self.name).or(Err(fmt::Error))?,
            std::str::from_utf8(&self.value).or(Err(fmt::Error))?,
        )
    }
}

impl Request {
    pub fn parse(buffer: Bytes) -> Result<Request, failure::Error> {
        let mut headers = [httparse::EMPTY_HEADER; 32];
        let mut req = httparse::Request::new(&mut headers);
        let len = req.parse(&buffer)?.unwrap();
        let request_line = RequestLine::from_parsed(&buffer, &req)?;
        let headers = req
            .headers
            .iter()
            .map(|header| Header {
                name: slice_to_bytes(&buffer, header.name.as_bytes()),
                value: slice_to_bytes(&buffer, header.value),
            })
            .collect();
        Ok(Request {
            request_line,
            headers,
            body: buffer.slice(len..),
        })
    }

    pub fn content_length(&self) -> Option<usize> {
        for header in self.headers.iter() {
            if header.name.as_ref() == b"Content-Length" {
                return std::str::from_utf8(&header.value)
                    .ok()
                    .and_then(|s| s.parse().ok());
            }
        }
        None
    }

    pub fn request_line(&self) -> &RequestLine {
        &self.request_line
    }

    pub fn method(&self) -> Method {
        self.request_line.method
    }

    pub fn uri(&self) -> &Uri {
        &self.request_line.uri
    }
}
