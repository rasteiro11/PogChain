use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint, PublicKey, SecretKey,
};
use rand_core::OsRng; // requires 'getrandom' feature
use sha2::{Digest, Sha256};

pub trait Hasher {
    fn hash(data: impl Into<String>) -> String;
}

pub struct SHA256;

impl SHA256 {
    pub fn new() -> Self {
        SHA256 {}
    }
}

struct KeyPair {
    public_key: PublicKey,
    private_key: SecretKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let secret = SecretKey::random(&mut OsRng {});
        let pk_bytes = EncodedPoint::from(secret.public_key());
        let public_key =
            PublicKey::from_sec1_bytes(pk_bytes.as_ref()).expect("Could not generate public key");
        Self {
            public_key,
            private_key: secret,
        }
    }
}

pub fn sign(private_key: &SecretKey, data: &String) -> Signature {
    let signing_key =
        SigningKey::from_bytes(private_key.to_be_bytes().as_ref()).expect("Could not sign message");
    signing_key.sign(data.as_bytes())
}

pub fn verify(public_key: &PublicKey, data: &String, signature: &Signature) -> bool {
    let verifying_key =
        VerifyingKey::from_affine(*public_key.as_affine()).expect("Verifying key failed");
    verifying_key.verify(data.as_bytes(), signature).is_ok()
}

impl Hasher for SHA256 {
    fn hash(data: impl Into<String>) -> String {
        format!("{:X}", Sha256::digest(data.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::{Hasher, SHA256};
    use crate::crypto::crypto::{sign, verify, KeyPair};

    #[test]
    fn test_hash() {
        let hash = SHA256::hash("Hello world!");
        assert_eq!(
            "C0535E4BE2B79FFD93291305436BF889314E4A3FAEC05ECFFCBB7DF31AD9E51A",
            hash
        )
    }

    #[test]
    fn test_sign() {
        let key_pair = KeyPair::new();

        let msg = "message to be signed".to_string();
        let msg_false = "message to be".to_string();

        let signature = sign(&key_pair.private_key, &msg);

        assert_eq!(true, verify(&key_pair.public_key, &msg, &signature));
        assert_eq!(false, verify(&key_pair.public_key, &msg_false, &signature))
    }
}
