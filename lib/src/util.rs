use crate::sha256::Hash;
use crate::types::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    pub fn calculate(transactions: &[Transaction]) -> Self {
        let mut layer: Vec<Hash> = vec![];
        for transaction in transactions {
            layer.push(Hash::hash(transaction));
        }
        if layer.len() > 1 {
            let mut new_layer: Vec<Hash> = vec![];
            for pair in layer.chunks(2) {
                let left_leaf = pair[0];
                let right_leaf = pair.get(1).unwrap_or(&pair[0]);
                new_layer.push(Hash::hash(&[left_leaf, *right_leaf]));
            }
            layer = new_layer;
        }

        MerkleRoot(layer[0])
    }
}
