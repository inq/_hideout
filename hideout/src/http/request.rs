use crate::{
    http::{Cookie, Uri},
    util::RcString,
};
use bytes::Bytes;
use std::fmt::{self, Debug};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid value: {}", value)]
    ValueError { value: String },
    #[fail(display = "httparse error")]
    Httparse,
    #[fail(display = "invalid http method")]
    HttpMethod,
    #[fail(display = "invalid http path")]
    HttpPath,
    #[fail(display = "invalid http version")]
    HttpVersion,
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

        let method = Method::from_str(parsed.method.ok_or(Error::HttpMethod)?)?;

        let uri = {
            use std::convert::TryFrom;

            let rc_string = RcString::from_utf8_unsafe(
                buffer.slice_ref(parsed.path.ok_or(Error::HttpMethod)?.as_bytes()),
            );
            Uri::try_from(rc_string)?
        };
        let version = parsed.version.ok_or(Error::HttpVersion)?.into();

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

#[derive(Debug)]
pub struct Request {
    request_line: RequestLine,
    headers: Vec<Header>,
    pub body: Bytes,
}

pub struct Header {
    name: RcString,
    value: RcString,
}

impl Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name.as_ref(), self.value.as_ref(),)
    }
}

impl Request {
    pub fn parse(buffer: Bytes) -> Result<Request, failure::Error> {
        let mut headers = [httparse::EMPTY_HEADER; 32];
        let mut req = httparse::Request::new(&mut headers);
        let len = if let httparse::Status::Complete(len) = req.parse(&buffer)? {
            len
        } else {
            return Err(Error::Httparse.into());
        };
        let request_line = RequestLine::from_parsed(&buffer, &req)?;
        let headers = req
            .headers
            .iter()
            .map(|header| -> Result<Header, failure::Error> {
                Ok(Header {
                    name: RcString::from_utf8_unsafe(buffer.slice_ref(header.name.as_bytes())),
                    value: RcString::from_utf8(buffer.slice_ref(header.value))?,
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Request {
            request_line,
            headers,
            body: buffer.slice(len..),
        })
    }

    pub fn cookie(&self) -> Cookie {
        let cookies_raw = self
            .headers
            .iter()
            .filter(|header| header.name.as_ref() == "Cookie")
            .map(|header| header.value.clone())
            .collect();
        Cookie::new(cookies_raw)
    }

    pub fn content_length(&self) -> Option<usize> {
        for header in self.headers.iter() {
            if header.name.as_ref() == "Content-Length" {
                return header.value.as_ref().parse().ok();
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
