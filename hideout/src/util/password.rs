use std::fmt;
struct HexBytes<'a>(&'a [u8]);

impl<'a> fmt::Display for HexBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

pub struct Password {
    hashed: String,
}

impl Password {
    pub fn new(orig: &str) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.input(orig);
        let result = hasher.result();
        Self {
            hashed: HexBytes(&result).to_string(),
        }
    }

    pub fn hashed(&self) -> String {
        self.hashed.clone()
    }
}
