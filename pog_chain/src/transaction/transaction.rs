use ed25519_dalek::PublicKey;

use super::{transaction_input::TransactionInput, transaction_output::TransactionOutput};

struct Transaction {
    transaction_id: String,
    sender: PublicKey,
    receiver: PublicKey,
    amount: f64,
    signature: String,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(
        sender: PublicKey,
        receiver: PublicKey,
        amount: f64,
        inputs: Vec<TransactionInput>,
    ) -> Self {
        Transaction {
            transaction_id: todo!(),
            sender,
            receiver,
            amount,
            signature: todo!(),
            inputs,
            outputs: Vec::new(),
        }
    }

    fn generate_signature(&mut self, private_key: :)

    fn calculate_hash(&mut self) {
        let hash_data = format!(
            "{}{}{}",
            String::from_utf8_lossy(self.sender.as_ref()),
            String::from_utf8_lossy(self.receiver.as_ref()),
            self.amount.trunc()
        );
        self.transaction_id = hash_data;
    }
}
