use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use crate::{
    config::config::{Config, CONFIG},
    crypto::crypto::{Hasher, SHA256},
    transaction::transaction::Transaction,
};

pub trait IBlock {
    fn increment_nonce(&mut self);
    fn get_prev_hash(&self) -> String;
    fn set_prev_hash(&mut self, s: String);
    fn get_hash(&self) -> String;
    fn set_hash(&mut self, s: &String);
    fn generate_hash(&mut self);
    fn get_id(&self) -> i64;
    fn add_transaction(&mut self, transaction: &Option<Transaction>) -> bool;
}

#[derive(Debug)]
pub struct Block {
    id: i64,
    nonce: i64,
    timestamp: i128,
    hash: String,
    previous_hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(previous_hash: String) -> Self {
        let block = Block {
            id: 0,
            nonce: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i128,
            hash: "".to_string(),
            previous_hash,
            transactions: vec![],
        };
        block.get_hash();
        block
    }
}

impl IBlock for Block {
    fn add_transaction(&mut self, transaction: &Option<Transaction>) -> bool {
        if let None = transaction {
            return false;
        }

        if !(self.previous_hash == CONFIG.get_genesis_prev_hash()) {
            if let Some(t) = &transaction {
                if !t.verify_transaction() {
                    println!("Transaction is not valid...");
                    return false;
                }
            }
        }

        self.transactions.push(transaction.unwrap());
        println!("Transaction is valid and it's added to block {}", self.id);
        true
    }

    fn increment_nonce(&mut self) {
        self.nonce += 1;
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

    fn generate_hash(&mut self) {
        let transactions_hash = "".to_string();
        for transaction in &self.transactions {
            transactions_hash.push_str(transaction.to_string().as_str());
        }

        let data = format!(
            "{}{}{}{}",
            self.id.to_string(),
            self.previous_hash,
            self.timestamp.to_string(),
            self.nonce.to_string(),
        );

        self.hash = SHA256::hash(data);
    }

    fn get_id(&self) -> i64 {
        return self.id;
    }
}

impl ToString for dyn IBlock {
    fn to_string(&self) -> String {
        format!(
            "ID: {}\n PREV HASH: {}\n HASH: {}\n",
            self.get_id().to_string(),
            self.get_prev_hash(),
            self.get_hash()
        )
    }
}
