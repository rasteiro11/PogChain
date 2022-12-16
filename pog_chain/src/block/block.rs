use std::time::{SystemTime, UNIX_EPOCH};

use crate::crypto::crypto::{Hasher, SHA256};

trait Hasheable {
    fn generate_hash(&mut self);
}

pub struct Block {
    id: i64,
    nonce: i64,
    timestamp: i128,
    hash: String,
    previous_hash: String,
    transaction: String,
}

impl Block {
    fn new(id: i64, transaction: String, previous_hash: String) -> Self {
        Block {
            id,
            nonce: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i128,
            hash: "".to_string(),
            previous_hash,
            transaction,
        }
    }

    fn increment_nonce(&mut self) {
        self.id += 1;
    }

    fn get_prev_hash(&self) -> String {
        self.previous_hash.clone()
    }

    fn set_prev_hash(&mut self, s: String) {
        self.previous_hash = s;
    }

    fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn set_hash(&mut self, s: &String) {
        self.hash = s.clone();
    }
}

impl Hasheable for Block {
    fn generate_hash(&mut self) {
        let data = format!(
            "{}{}{}{}{}",
            self.id.to_string(),
            self.previous_hash,
            self.timestamp.to_string(),
            self.nonce.to_string(),
            self.transaction.to_string()
        );

        self.hash = SHA256::hash(data);
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.id.to_string(),
            self.previous_hash,
            self.timestamp.to_string(),
            self.nonce.to_string(),
            self.transaction.to_string()
        )
    }
}
