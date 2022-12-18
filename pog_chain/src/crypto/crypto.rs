use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
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

pub fn sign(private_key: &SigningKey, data: &String) -> Signature {
    private_key.sign(data.as_bytes())
}

pub fn verify(public_key: &SigningKey, data: &String, signature: Signature) -> bool {
    let verifying_key = VerifyingKey::from(public_key);
    verifying_key.verify(data.as_bytes(), &signature).is_ok()
}

impl Hasher for SHA256 {
    fn hash(data: impl Into<String>) -> String {
        format!("{:X}", Sha256::digest(data.into()))
    }
}

#[cfg(test)]
mod tests {

    use ed25519_dalek::{Keypair, PublicKey};
    use elliptic_curve::{group::GroupEncoding, AffineXCoordinate};
    use p256::{
        ecdh::EphemeralSecret,
        ecdsa::{signature::Signer, SigningKey, VerifyingKey},
    };
    use rand_os::OsRng;

    use super::{Hasher, SHA256};

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
        //let mut csprng = OsRng {};
        //let keypair = Keypair::generate(&mut csprng);

        //println!(
        //    "PUBLIC KEY: {:?}",
        //    base64::encode(keypair.public.to_bytes())
        //);
        //println!(
        //    "PRIVATE KEY: {:?}",
        //    base64::encode(base64::encode(keypair.secret.to_bytes()))
        //);

        //use ed25519_dalek::{Signature, Signer};
        //let message: &[u8] = b"This is a test of the tsunami alert system.";
        //let signature: Signature = keypair.sign(message);

        //use ed25519_dalek::Verifier;
        //assert!(keypair.verify(message, &signature).is_ok());

        //let public_key: PublicKey = keypair.public;
        //assert!(public_key.verify(message, &signature).is_ok());

        //use p256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
        //use rand_core::OsRng; // requires 'getrandom' feature

        //let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        //let message =
        //    b"ECDSA proves knowledge of a secret number in the context of a single message";
        //let signature = signing_key.sign(message);
        //let verifying_key = VerifyingKey::from(&signing_key);

        // Alice
        //let alice_secret = EphemeralSecret::random(&mut OsRng);
        //let alice_public_key_bytes = EncodedPoint::from(alice_secret.public_key());

        ////Bob
        //let bob_secret = EphemeralSecret::random(&mut OsRng);
        //let bob_pk_bytes = EncodedPoint::from(bob_secret.public_key());

        //// Alice decodes Bob's serialized public key and computes a shared secret from it
        //let bob_public = PublicKey::from_sec1_bytes(bob_pk_bytes.as_ref())
        //    .expect("bob's public key is invalid!"); // In real usage, don't panic, handle this!

        //let alice_shared = alice_secret.diffie_hellman(&bob_public);

        //// Bob decodes Alice's serialized public key and computes the same shared secret
        //let alice_public = PublicKey::from_sec1_bytes(alice_public_key_bytes.as_ref())
        //    .expect("alice's public key is invalid!"); // In real usage, don't panic, handle this!

        //let bob_shared = bob_secret.diffie_hellman(&alice_public);

        //let shared_secret = elliptic_curve::ecdh::diffie_hellman(
        //    alice_pk_bytes.to_nonzero_scalar(),
        //    alice_secret.public_key().as_affine(),
        //);
    }
}
