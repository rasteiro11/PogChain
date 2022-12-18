use ed25519_dalek::Keypair;
use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint, PublicKey, SecretKey,
};

use crate::{crypto::crypto::KeyPair, transaction::transaction::Transaction};

struct Wallet {
    // signature
    private_key: SecretKey,
    // verification
    public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let key_pair = KeyPair::new();
        Self {
            private_key: key_pair.get_private_key(),
            public_key: key_pair.get_public_key(),
        }
    }

    fn transfer_money(&self, receiver: &PublicKey, amount: f64) -> &Option<Transaction> {
        if self.calculate_balance() < amount {
            println!("Invalid trnsaction because of not enough money");
            return &None;
        }

        let inputs = vec![];
    }

    fn calculate_balance(&self) -> f64 {
        let balance = 0.0;
        balance
    }
}
