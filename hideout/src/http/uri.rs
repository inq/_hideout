use bytes::Bytes;
use failure::Fail;

#[derive(Debug)]
pub enum State {
    Initial,
    PathToken(usize),
    QueryKey(usize),
    QueryValue(usize, usize),
    Fragment(usize),
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid state: {:?}, {:?} at {}", state, chr, at)]
    InvalidState { state: State, chr: char, at: usize },
}

#[derive(Debug)]
pub struct Uri {
    path: Vec<Bytes>,
    query: Vec<(Bytes, Option<Bytes>)>,
    fragment: Option<Bytes>,
}

impl Uri {
    pub fn from_bytes(bytes: &Bytes) -> Result<Self, Error> {
        let data = unsafe { std::str::from_utf8_unchecked(&bytes) };
        let mut state = State::Initial;
        let mut path = vec![];
        let mut query = vec![];
        let mut fragment = None;

        for (idx, chr) in data.char_indices() {
            state = match (state, chr) {
                (State::Initial, '/') => State::PathToken(idx + 1),
                (State::PathToken(start_at), chr @ ('/' | '?' | '#')) => {
                    let token = bytes.slice(start_at..idx);
                    path.push(token);
                    match chr {
                        '/' => State::PathToken(idx + 1),
                        '?' => State::QueryKey(idx + 1),
                        '#' => State::Fragment(idx + 1),
                        _ => unreachable!(),
                    }
                }
                (State::QueryKey(start_at), chr @ ('&' | '#')) => {
                    let key = bytes.slice(start_at..idx);
                    query.push((key, None));
                    match chr {
                        '&' => State::QueryKey(idx + 1),
                        '#' => State::Fragment(idx + 1),
                        _ => unreachable!(),
                    }
                }
                (State::QueryKey(start_at), '=') => State::QueryValue(start_at, idx + 1),
                (State::QueryValue(key_start_at, value_start_at), chr @ ('&' | '#')) => {
                    let key = bytes.slice(key_start_at..value_start_at - 1);
                    let value = bytes.slice(value_start_at..idx);
                    query.push((key, Some(value)));
                    match chr {
                        '&' => State::QueryKey(idx + 1),
                        '#' => State::Fragment(idx + 1),
                        _ => unreachable!(),
                    }
                }
                (state, _) => state,
            }
        }
        match state {
            State::Initial => (),
            State::PathToken(start_at) => {
                let token = bytes.slice(start_at..);
                path.push(token);
            }
            State::QueryKey(start_at) => {
                let key = bytes.slice(start_at..);
                query.push((key, None));
            }
            State::QueryValue(key_start_at, value_start_at) => {
                let key = bytes.slice(key_start_at..value_start_at - 1);
                let value = bytes.slice(value_start_at..);
                query.push((key, Some(value)));
            }
            State::Fragment(start_at) => fragment = Some(bytes.slice(start_at..)),
        }
        Ok(Uri {
            path,
            query,
            fragment,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bytes = Bytes::from_static(b"/hello/world?param=?&?=?#fragment=#123");
        let uri = Uri::from_bytes(&bytes).unwrap();
        assert!(
            matches!(uri.fragment, Some(fragment) if fragment == Bytes::from_static(b"fragment=#123"))
        );
    }
}
