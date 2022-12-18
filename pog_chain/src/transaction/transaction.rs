use std::hash;

use elliptic_curve::{group::GroupEncoding, SecretKey};
use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint, NistP256, PublicKey,
};

use crate::crypto::{
    self,
    crypto::{sign, verify},
};

use super::{
    transaction_input::{self, TransactionInput},
    transaction_output::TransactionOutput,
};

struct Transaction {
    sender: PublicKey,
    receiver: PublicKey,
    amount: f64,
    signature: Option<Signature>,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    transaction_id: String,
}

impl Transaction {
    pub fn new(
        sender: PublicKey,
        receiver: PublicKey,
        amount: f64,
        inputs: Vec<TransactionInput>,
    ) -> Self {
        let mut transaction = Transaction {
            transaction_id: "".into(),
            sender,
            receiver,
            amount,
            signature: None,
            inputs,
            outputs: Vec::new(),
        };
        transaction.calculate_hash();
        transaction
    }

    pub fn verify_transaction(&mut self) -> bool {
        if !self.verify_signature() {
            println!("Invalid transaction because of invalid signature");
            return false;
        }

        let mut sum = 0.0;

        for transaction_input in &self.inputs {
            //transaction_input.set_utxo(transaction_output)
            //if let Some(to) = transaction_input.get_utxo() {
            //    sum += to.get_amount()
            //}
        }
        self.outputs.push(TransactionOutput::new(
            self.receiver,
            self.amount,
            &self.transaction_id,
        ));

        self.outputs.push(TransactionOutput::new(
            self.sender,
            self.get_inputs_sum() - self.amount,
            &self.transaction_id,
        ));

        true
    }

    fn get_inputs_sum(&self) -> f64 {
        let mut sum = 0.0;

        for transaction_input in &self.inputs {
            if let Some(to) = transaction_input.get_utxo() {
                sum += to.get_amount()
            }
        }
        sum
    }

    fn generate_signature(&mut self, private_key: SecretKey<NistP256>) {
        let hash_data = format!(
            "{}{}{}",
            String::from_utf8_lossy(self.sender.as_ref().to_bytes().as_ref()),
            String::from_utf8_lossy(self.receiver.as_ref().to_bytes().as_ref()),
            self.amount.to_string()
        );
        self.signature = Some(sign(&private_key, &hash_data));
    }

    fn verify_signature(&mut self) -> bool {
        let hash_data = format!(
            "{}{}{}",
            String::from_utf8_lossy(self.sender.as_ref().to_bytes().as_ref()),
            String::from_utf8_lossy(self.receiver.as_ref().to_bytes().as_ref()),
            self.amount.to_string()
        );
        if let Some(signature) = self.signature {
            verify(&self.sender, &hash_data, &signature)
        } else {
            false
        }
    }
    fn calculate_hash(&mut self) {
        let hash_data = format!(
            "{}{}{}",
            String::from_utf8_lossy(self.sender.as_ref().to_bytes().as_ref()),
            String::from_utf8_lossy(self.receiver.as_ref().to_bytes().as_ref()),
            self.amount.to_string()
        );
        self.transaction_id = hash_data;
    }
}
