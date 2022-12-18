use ed25519_dalek::PublicKey;

use crate::crypto::crypto::{Hasher, SHA256};

pub struct TransactionOutput {
    id: String,
    parent_transaction_id: String,
    receiver: PublicKey,
    amount: f64,
}

impl TransactionOutput {
    pub fn new(receiver: PublicKey, amount: f64, parent_transaction_id: String) -> Self {
        Self {
            id: SHA256::hash(format!(
                "{}{}{}",
                base64::encode(receiver.as_ref()),
                amount.to_string(),
                parent_transaction_id
            )),
            parent_transaction_id,
            receiver,
            amount,
        }
    }

    fn is_mine(&self, public_key: &PublicKey) -> bool {
        String::from_utf8_lossy(public_key.as_ref())
            == String::from_utf8_lossy(self.receiver.as_ref())
    }
}
