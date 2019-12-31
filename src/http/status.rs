use std::fmt;

pub struct StatusCode(pub u16);

fn reason_phrase(code: &StatusCode) -> Option<&'static str> {
    let res = match code.0 {
        100 => "Continue",
        101 => "Switching Protocols",

        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",

        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        _ => return None,
    };
    Some(res)
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(reason) = reason_phrase(self) {
            write!(f, "{} {}", self.0, reason)
        } else {
            Err(fmt::Error)
        }
    }
}
