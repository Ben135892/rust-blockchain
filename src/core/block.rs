use super::transaction::{new_transaction, Transaction};
use crate::{crypto::keypair::KeyPair, types::hash::Hash};
use serde::{Deserialize, Serialize};
use sha256::{digest, try_digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub version: u32,
    pub data_hash: Hash,
    pub timestamp: i64,
    pub prev_block_hash: Hash,
    pub height: u32,
    #[serde(skip_serializing)]
    hash: Option<Hash>,
}

pub fn new_header(
    version: u32,
    data_hash: Hash,
    prev_block_hash: Hash,
    timestamp: i64,
    height: u32,
) -> Header {
    Header {
        version,
        data_hash,
        timestamp,
        prev_block_hash,
        height,
        hash: None,
    }
}

impl Header {
    pub fn encode(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn hash(&mut self) -> Hash {
        if self.hash.clone() == None {
            self.hash = Some(digest(self.encode()));
        }
        self.hash.clone().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

pub fn new_block(header: Header, transactions: Vec<Transaction>) -> Block {
    Block {
        header: header,
        transactions: transactions,
    }
}

pub fn new_block_from_prev_header(
    prev_header: &mut Header,
    mut transactions: Vec<Transaction>,
) -> Block {
    let data_hash = calculate_data_hash(&mut transactions);
    let start = SystemTime::now();
    let since = start.duration_since(UNIX_EPOCH).expect("time error");
    let header = Header {
        version: 0,
        data_hash,
        timestamp: since.as_millis() as i64,
        height: prev_header.height + 1,
        prev_block_hash: prev_header.hash(),
        hash: None,
    };
    return new_block(header, transactions);
}

pub fn calculate_data_hash(transactions: &mut Vec<Transaction>) -> Hash {
    let mut result = "".to_string();
    for t in transactions {
        result += &t.hash();
    }
    digest(result)
}

impl Block {
    pub fn encode(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn hash(&mut self) -> Hash {
        self.header.hash()
    }

    pub fn verify(&mut self) -> Result<(), String> {
        for t in &self.transactions {
            t.verify()?;
        }
        // verify data hash matches
        let data_hash = calculate_data_hash(&mut self.transactions);
        if data_hash != self.header.data_hash {
            return Err(format!("block {} has an invalid data hash", self.hash()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_block() {
        let mut b = random_block(0, "".to_string());

        let data = b.encode();
        println!("{}", data);
        let mut b_decode: Block = serde_json::from_str(&data).unwrap();

        assert_eq!(b.hash() == b_decode.hash(), true);
    }

    #[test]
    fn test_verify_block() {
        let mut b = random_block(0, "".to_string());

        assert_eq!(b.verify().is_ok(), true);

        let other_tx = new_transaction([0; 20], 5);
        // don't sign transaction
        b.transactions.push(other_tx);

        assert_eq!(b.verify().is_ok(), false);

        // remove transaction
        b.transactions.pop();
        assert_eq!(b.verify().is_ok(), true);

        b.header.data_hash = "invalid hash".to_string();
        assert_eq!(b.verify().is_ok(), false);
    }
}

pub fn random_block(height: u32, prev_hash: Hash) -> Block {
    let mut tx = new_transaction([0; 20], 5);
    let key_pair = KeyPair::new(0);
    tx.sign(&key_pair);
    let header = new_header(0, "".to_string(), prev_hash, 0, 0);
    let mut b = new_block(header, vec![tx]);
    b.header.data_hash = calculate_data_hash(&mut b.transactions);
    b
}
