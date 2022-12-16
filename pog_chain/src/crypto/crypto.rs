use sha2::{Digest, Sha256};

pub trait Hasher {
    fn hash(&self, data: impl Into<String>) -> String;
}

pub struct SHA256;

impl SHA256 {
    pub fn new() -> Self {
        SHA256 {}
    }
}

impl Hasher for SHA256 {
    fn hash(&self, data: impl Into<String>) -> String {
        format!("{:X}", Sha256::digest(data.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::{Hasher, SHA256};

    #[test]
    fn test_hash() {
        let hash = SHA256::new().hash("Hello world!");
        println!("{}", hash);
        assert_eq!(
            "C0535E4BE2B79FFD93291305436BF889314E4A3FAEC05ECFFCBB7DF31AD9E51A",
            hash
        )
    }
}
