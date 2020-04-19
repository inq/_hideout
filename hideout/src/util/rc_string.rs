use std::{borrow, fmt, hash, ops};

#[derive(Clone)]
pub struct RcString {
    inner: bytes::Bytes,
}

impl AsRef<str> for RcString {
    #[inline]
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

impl ops::Deref for RcString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

impl hash::Hash for RcString {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        (*self.as_ref()).hash(hasher)
    }
}

impl PartialEq for RcString {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for RcString {}

impl borrow::Borrow<str> for RcString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Debug for RcString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl RcString {
    #[inline]
    pub fn from_utf8(bytes: bytes::Bytes) -> Result<Self, std::str::Utf8Error> {
        let _ = std::str::from_utf8(&bytes)?;
        Ok(Self { inner: bytes })
    }

    #[inline]
    pub fn from_utf8_unsafe(bytes: bytes::Bytes) -> Self {
        Self { inner: bytes }
    }

    #[inline]
    pub fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            inner: self.inner.slice(range),
        }
    }

    #[inline]
    pub fn slice_ref(&self, bytes: &str) -> Self {
        Self {
            inner: self.inner.slice_ref(bytes.as_bytes()),
        }
    }
}
