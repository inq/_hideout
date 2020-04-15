use crate::util::RcString;
use failure::Fail;
use std::convert;

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
    inner: RcString,
    path: Vec<RcString>,
    query: Vec<(RcString, Option<RcString>)>,
    fragment: Option<RcString>,
}

impl Uri {
    pub fn len_path(&self) -> usize {
        self.path.len()
    }

    pub fn nth_path(&self, idx: usize) -> Option<&str> {
        self.path.get(idx).map(std::convert::AsRef::as_ref)
    }
}

impl convert::AsRef<str> for Uri {
    #[inline]
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl convert::TryFrom<RcString> for Uri {
    type Error = Error;

    fn try_from(value: RcString) -> Result<Self, Self::Error> {
        let mut state = State::Initial;
        let mut path = vec![];
        let mut query = vec![];
        let mut fragment = None;

        for (idx, chr) in value.as_ref().char_indices() {
            state = match (state, chr) {
                (State::Initial, '/') => State::PathToken(idx + 1),
                (State::PathToken(start_at), chr @ ('/' | '?' | '#')) => {
                    let token = value.slice(start_at..idx);
                    path.push(token);
                    match chr {
                        '/' => State::PathToken(idx + 1),
                        '?' => State::QueryKey(idx + 1),
                        '#' => State::Fragment(idx + 1),
                        _ => unreachable!(),
                    }
                }
                (State::QueryKey(start_at), chr @ ('&' | '#')) => {
                    let key = value.slice(start_at..idx);
                    query.push((key, None));
                    match chr {
                        '&' => State::QueryKey(idx + 1),
                        '#' => State::Fragment(idx + 1),
                        _ => unreachable!(),
                    }
                }
                (State::QueryKey(start_at), '=') => State::QueryValue(start_at, idx + 1),
                (State::QueryValue(key_start_at, value_start_at), chr @ ('&' | '#')) => {
                    let key = value.slice(key_start_at..value_start_at - 1);
                    let value = value.slice(value_start_at..idx);
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
                let token = value.slice(start_at..);
                if !token.as_ref().is_empty() {
                    path.push(token);
                }
            }
            State::QueryKey(start_at) => {
                let key = value.slice(start_at..);
                query.push((key, None));
            }
            State::QueryValue(key_start_at, value_start_at) => {
                let key = value.slice(key_start_at..value_start_at - 1);
                let value = value.slice(value_start_at..);
                query.push((key, Some(value)));
            }
            State::Fragment(start_at) => fragment = Some(value.slice(start_at..)),
        }
        Ok(Uri {
            inner: value,
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
        use std::convert::TryFrom;

        let bytes = bytes::Bytes::from_static(b"/hello/world?param=?&?=?#fragment=#123");
        let rc_string = RcString::from_utf8(bytes).unwrap();
        let uri = Uri::try_from(rc_string).unwrap();
        assert!(matches!(uri.fragment, Some(fragment) if fragment.as_ref() == "fragment=#123"));
    }
}
