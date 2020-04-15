use std::fmt;

pub struct RcString {
    inner: bytes::Bytes,
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
}

impl AsRef<str> for RcString {
    #[inline]
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}
