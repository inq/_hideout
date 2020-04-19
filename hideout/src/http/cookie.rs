use crate::util::RcString;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cookie {
    inner: HashMap<RcString, RcString>,
}

impl Cookie {
    pub fn new(raw: Vec<RcString>) -> Self {
        #[allow(clippy::mutable_key_type)]
        let mut inner = HashMap::new();
        for line in raw.iter() {
            for token in line.as_ref().split("; ") {
                if let Some(splitter) = token.find('=') {
                    let key = line.slice_ref(&token[..splitter]);
                    let value = line.slice_ref(&token[splitter + 1..]);
                    inner.insert(key, value);
                }
            }
        }
        Self { inner }
    }

    pub fn get(&self, key: &str) -> Option<RcString> {
        self.inner.get(key).cloned()
    }
}
