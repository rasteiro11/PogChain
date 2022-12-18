use super::transaction_output::TransactionOutput;

pub struct TransactionInput {
    transaction_output_id: String,
    utxo: Option<TransactionOutput>,
}

impl TransactionInput {
    pub fn new(transaction_output_id: &String) -> Self {
        Self {
            utxo: None,
            transaction_output_id: transaction_output_id.clone(),
        }
    }

    fn get_transaction_output_id(&self) -> &String {
        &self.transaction_output_id
    }

    pub fn get_utxo(&self) -> &Option<TransactionOutput> {
        &self.utxo
    }
    pub fn set_utxo(&mut self, transaction_output: TransactionOutput) {
        self.utxo = Some(transaction_output);
    }
}
