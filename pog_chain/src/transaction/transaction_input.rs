use super::transaction_output::TransactionOutput;

pub struct TransactionInput {
    transaction_output_id: String,
    utxo: TransactionOutput,
}

impl TransactionInput {
    pub fn new(transaction_output_id: &String) -> Self {
        Self {
            utxo: TransactionOutput {},
            transaction_output_id: transaction_output_id.clone(),
        }
    }

    fn get_transaction_output_id(&self) -> &String {
        &self.transaction_output_id
    }

    fn get_utxo(&self) -> &TransactionOutput {
        &self.utxo
    }
}
