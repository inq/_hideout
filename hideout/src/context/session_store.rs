use std::{
    borrow::Borrow, cell::RefCell, collections::HashMap, convert::AsRef, hash::Hash, rc::Rc,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Key(String);

impl Key {
    pub fn new(inner: &str) -> Self {
        Self(inner.to_string())
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct KeyRef<'a>(&'a str);

impl<'a> AsRef<str> for KeyRef<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct Inner<T> {
    map: HashMap<Key, T>,
}

pub struct SessionStore<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl<T> std::clone::Clone for SessionStore<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Default for SessionStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SessionStore<T> {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner {
                map: HashMap::default(),
            })),
        }
    }

    pub fn set<K>(&self, key: K, value: T) -> Option<T>
    where
        K: AsRef<Key>,
    {
        self.inner
            .borrow_mut()
            .map
            .insert(key.as_ref().clone(), value)
    }

    pub fn get<Q>(&self, key: &Q)
    where
        Key: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.borrow_mut().map.get(key);
    }
}