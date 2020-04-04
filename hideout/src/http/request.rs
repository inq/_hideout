use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{rest, value};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

use bytes::Bytes;
use std::fmt::{self, Debug};

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

#[derive(Clone, Debug)]
pub enum Version {
    Http10,
    Http11,
}

pub struct RequestLine {
    method: Method,
    uri: Bytes,
    version: Version,
}

impl Debug for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RequestLine {{ {:?} {:?} {:?} }}",
            self.method,
            std::str::from_utf8(&self.uri).or(Err(fmt::Error))?,
            self.version
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

fn is_token(c: u8) -> bool {
    match c {
        128..=255 => false,
        0..=31 => false,
        b'(' => false,
        b')' => false,
        b'<' => false,
        b'>' => false,
        b'@' => false,
        b',' => false,
        b';' => false,
        b':' => false,
        b'\\' => false,
        b'"' => false,
        b'/' => false,
        b'[' => false,
        b']' => false,
        b'?' => false,
        b'=' => false,
        b'{' => false,
        b'}' => false,
        b' ' => false,
        _ => true,
    }
}

fn is_not_space(c: u8) -> bool {
    c != b' '
}

fn is_horizontal_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn not_cr(c: u8) -> bool {
    c != b'\r'
}

fn version(i: &[u8]) -> IResult<&[u8], Version> {
    let (input, _) = tag(" HTTP/")(i)?;
    let (input, version) = alt((
        value(Version::Http10, tag("1.0")),
        value(Version::Http11, tag("1.1")),
    ))(input)?;
    Ok((input, version))
}

fn request_line<'a>(buffer: &Bytes, input: &'a [u8]) -> IResult<&'a [u8], RequestLine> {
    let (input, method) = alt((
        value(Method::Get, tag("GET ")),
        value(Method::Post, tag("POST ")),
        value(Method::Put, tag("PUT ")),
        value(Method::Delete, tag("DELETE ")),
        value(Method::Options, tag("OPTIONS ")),
        value(Method::Trace, tag("TRACE ")),
        value(Method::Connect, tag("CONNECT ")),
        value(Method::Head, tag("HEAD")),
    ))(input)?;
    let (input, uri) = take_while1(is_not_space)(input)?;
    let (input, version) = version(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((
        input,
        RequestLine {
            method,
            uri: slice_to_bytes(buffer, uri),
            version,
        },
    ))
}

#[derive(Debug)]
pub struct Request {
    request_line: RequestLine,
    headers: Vec<Header>,
    pub body: Bytes,
}

pub struct RawHeader<'a> {
    name: &'a [u8],
    value: &'a [u8],
}

impl<'a> RawHeader<'a> {
    fn build(self, buffer: &Bytes) -> Header {
        Header {
            name: slice_to_bytes(buffer, &self.name),
            value: slice_to_bytes(buffer, &self.value),
        }
    }
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

fn header<'a>(input: &'a [u8]) -> IResult<&'a [u8], RawHeader<'a>> {
    let (input, name) = take_while1(is_token)(input)?;
    let (input, _) = nom::character::complete::char(':')(input)?;
    let (input, _) = take_while1(is_horizontal_space)(input)?;
    let (input, value) = take_while1(not_cr)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, RawHeader { name, value }))
}

fn request<'a>(buffer: &Bytes, input: &'a [u8]) -> IResult<&'a [u8], Request> {
    let (input, request_line) = request_line(&buffer, input)?;
    let (input, raw_headers) = terminated(many1(header), tag("\r\n"))(input)?;
    let (input, body) = rest(input)?;

    Ok((
        input,
        Request {
            request_line,
            headers: raw_headers
                .into_iter()
                .map(|raw| raw.build(&buffer))
                .collect(),
            body: slice_to_bytes(buffer, body),
        },
    ))
}

impl Request {
    pub fn parse(buffer: Bytes) -> Option<Request> {
        let input = &buffer;
        match request(&buffer, input) {
            Ok((_, output)) => Some(output),
            _ => None,
        }
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

    pub fn uri(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.request_line.uri)
    }
}
