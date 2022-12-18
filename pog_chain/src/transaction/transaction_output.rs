use elliptic_curve::group::GroupEncoding;
use p256::PublicKey;

use crate::crypto::crypto::{Hasher, SHA256};

pub struct TransactionOutput {
    id: String,
    parent_transaction_id: String,
    receiver: PublicKey,
    amount: f64,
}

impl TransactionOutput {
    pub fn new(receiver: PublicKey, amount: f64, parent_transaction_id: &String) -> Self {
        let pt_id = parent_transaction_id.clone();
        Self {
            id: SHA256::hash(format!(
                "{}{}{}",
                String::from_utf8_lossy(receiver.as_ref().to_bytes().as_ref()),
                amount.to_string(),
                parent_transaction_id
            )),
            parent_transaction_id: pt_id,
            receiver,
            amount,
        }
    }

    fn is_mine(&self, public_key: &PublicKey) -> bool {
        String::from_utf8_lossy(public_key.as_ref().to_bytes().as_ref())
            == String::from_utf8_lossy(self.receiver.as_ref().to_bytes().as_ref())
    }

    pub fn get_amount(&self) -> f64 {
        return self.amount;
    }
}
