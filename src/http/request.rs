use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{rest, value};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

use std::fmt::{self, Debug};

#[derive(Clone, Debug)]
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

pub struct RequestLine<'a> {
    method: Method,
    uri: &'a [u8],
    version: Version,
}

impl<'a> Debug for RequestLine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RequestLine {{ {:?} {:?} {:?} }}",
            self.method,
            std::str::from_utf8(self.uri).or(Err(fmt::Error))?,
            self.version
        )
    }
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

fn request_line<'a>(input: &'a [u8]) -> IResult<&'a [u8], RequestLine<'a>> {
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
            uri,
            version,
        },
    ))
}

#[derive(Debug)]
pub struct Request<'a> {
    request_line: RequestLine<'a>,
    headers: Vec<Header<'a>>,
    body: &'a [u8],
}

pub struct Header<'a> {
    name: &'a [u8],
    value: &'a [u8],
}

impl<'a> Debug for Header<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?}",
            std::str::from_utf8(self.name).or(Err(fmt::Error))?,
            std::str::from_utf8(self.value).or(Err(fmt::Error))?,
        )
    }
}

fn header<'a>(input: &'a [u8]) -> IResult<&'a [u8], Header<'a>> {
    let (input, name) = take_while1(is_token)(input)?;
    let (input, _) = nom::character::complete::char(':')(input)?;
    let (input, _) = take_while1(is_horizontal_space)(input)?;
    let (input, value) = take_while1(not_cr)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Header { name, value }))
}

fn request<'a>(input: &'a [u8]) -> IResult<&'a [u8], Request<'a>> {
    let (input, request_line) = request_line(input)?;
    let (input, headers) = terminated(many1(header), tag("\r\n"))(input)?;
    let (input, body) = rest(input)?;

    Ok((
        input,
        Request {
            request_line,
            headers,
            body,
        },
    ))
}

impl<'a> Request<'a> {
    pub fn parse(buffer: &'a [u8]) -> Option<Request<'a>> {
        match request(buffer) {
            Ok((_, output)) => Some(output),
            _ => None,
        }
    }
}
