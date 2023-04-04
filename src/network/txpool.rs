use std::collections::HashMap;

use crate::{core::transaction::Transaction, types::hash::Hash};

pub struct TxPool {
    transactions: HashMap<Hash, Transaction>,
}

impl TxPool {
    pub fn new() -> Self {
        TxPool {
            transactions: HashMap::new(),
        }
    }

    pub fn add(&mut self, mut tx: Transaction) {
        self.transactions.insert(tx.hash(), tx);
    }

    pub fn transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    pub fn has(&self, tx: &mut Transaction) -> bool {
        self.transactions.contains_key(&tx.hash())
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }
}
