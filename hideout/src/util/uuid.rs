use std::fmt;

pub struct Uuid {
    inner: [u8; 16],
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.inner[0..4].iter() {
            write!(f, "{:02x}", c)?;
        }
        write!(f, "-")?;
        for c in self.inner[4..6].iter() {
            write!(f, "{:02x}", c)?;
        }
        write!(f, "-")?;
        for c in self.inner[6..8].iter() {
            write!(f, "{:02x}", c)?;
        }
        write!(f, "-")?;
        for c in self.inner[8..10].iter() {
            write!(f, "{:02x}", c)?;
        }
        write!(f, "-")?;
        for c in self.inner[10..16].iter() {
            write!(f, "{:02x}", c)?;
        }
        Ok(())
    }
}

impl Uuid {
    pub fn new_v4(rng: &mut impl rand::Rng) -> Self {
        let inner = rng.gen::<[u8; 16]>();
        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let uuid = Uuid::new_v4(&mut rng);
        assert_eq!(uuid.to_string().len(), 36);
    }
}
