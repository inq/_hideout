#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Trace,
    Connect,
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    url: String,
    version: String,
}

impl Request {
    pub fn parse(i: &[u8]) -> Self {
        request_line(i).unwrap().1
    }
}

// https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html#sec5.1
fn request_line(i: &[u8]) -> nom::IResult<&[u8], Request> {
    use nom::{
        bytes::complete::{tag, take_while1},
        sequence::{preceded, tuple},
        AsChar,
    };

    let http = tag("HTTP/");
    let is_version = |c: u8| c >= b'0' && c <= b'9' || c == b'.';
    let version = take_while1(is_version);
    let http_version = preceded(http, version);
    let method = take_while1(AsChar::is_alpha);
    let space = take_while1(|c| c == b' ');
    let url = take_while1(|c| c != b' ');
    let line_ending = tag("\r\n");

    let t = tuple((method, &space, url, &space, http_version, line_ending));
    let (input, (method, _, url, _, version, _)) = t(i)?;

    let method = match method {
        b"GET" => Method::Get,
        b"POST" => Method::Post,
        b"PUT" => Method::Put,
        b"DELETE" => Method::Delete,
        b"OPTIONS" => Method::Options,
        b"HEAD" => Method::Head,
        b"TRACE" => Method::Trace,
        b"CONNECT" => Method::Connect,
        _ => panic!(),
    };
    let url = String::from_utf8(url.to_vec()).unwrap();
    let version = String::from_utf8(version.to_vec()).unwrap();

    Ok((
        input,
        Request {
            method,
            url,
            version,
        },
    ))
}
