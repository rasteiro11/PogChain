use std::collections::LinkedList;

use crate::{
    block::block::IBlock,
    crypto::crypto::{Hasher, SHA256},
};

pub struct MerkleTree {
    transactions: Vec<String>,
}

impl MerkleTree {
    pub fn new(transactions: Vec<String>) -> Self {
        MerkleTree { transactions }
    }

    pub fn get_merkle_root(&self) -> String {
        return self.construct(&self.transactions)[0].clone();
    }

    fn construct(&self, transactions: &Vec<String>) -> Vec<String> {
        if transactions.len() == 1 {
            return transactions.to_owned();
        }

        let mut updated_list = vec![];
        let mut i = 0;
        while i < transactions.len() - 1 {
            updated_list.push(self.merge_hash(&transactions[i], &transactions[i + 1]));
            i += 2;
        }

        if (transactions.len() % 2) == 1 {
            updated_list.push(self.merge_hash(
                &transactions[transactions.len() - 1],
                &transactions[transactions.len() - 1],
            ))
        }

        self.construct(&updated_list)
    }

    fn merge_hash(&self, hash1: &String, hash2: &String) -> String {
        return SHA256::hash(format!("{}{}", hash1, hash2));
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTree;
    use crate::crypto::crypto::{Hasher, SHA256};

    #[test]
    fn test_merkle_tree_even() {
        let mut transactions = Vec::new();

        transactions.push("aa".into());
        transactions.push("bb".into());
        transactions.push("dd".into());
        transactions.push("ee".into());

        let merkle_tree = MerkleTree::new(transactions);

        let hash1 = SHA256::hash(format!("{}{}", "aa", "bb"));
        let hash2 = SHA256::hash(format!("{}{}", "dd", "ee"));
        let final_hash = SHA256::hash(format!("{}{}", hash1, hash2));

        assert_eq!(merkle_tree.get_merkle_root(), final_hash);
    }

    #[test]
    fn test_merkle_tree_odd() {
        let mut transactions = Vec::new();

        transactions.push("aa".into());
        transactions.push("bb".into());
        transactions.push("dd".into());

        let merkle_tree = MerkleTree::new(transactions);

        let hash1 = SHA256::hash(format!("{}{}", "aa", "bb"));
        let hash2 = SHA256::hash(format!("{}{}", "dd", "dd"));
        let final_hash = SHA256::hash(format!("{}{}", hash1, hash2));

        assert_eq!(merkle_tree.get_merkle_root(), final_hash);
    }
}
