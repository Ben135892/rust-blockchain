use crate::crypto::keypair::{new_pk_from_string, new_sig_from_string};
use crate::types::hash::Hash;
use crate::{crypto::keypair::KeyPair, types::address::Address};
use secp256k1::{
    rand::{rngs, SeedableRng},
    Message, PublicKey, SecretKey, Signature,
};
use serde::{Deserialize, Serialize};
use sha256::{digest, try_digest};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub public_key: Option<String>,
    pub signature: Option<String>,

    pub data: Data,

    #[serde(skip_serializing)]
    hash: Option<Hash>,

    #[serde(skip_serializing)]
    first_seen: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    to: Address,
    amount: u64,
}

impl Transaction {
    pub fn new(to: Address, amount: u64) -> Self {
        Self {
            data: Data { to, amount },
            public_key: None,
            signature: None,
            hash: None,
            first_seen: None,
        }
    }

    pub fn encode(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn encode_for_hash(&self) -> String {
        serde_json::to_string(&self.data).unwrap()
    }

    pub fn hash(&mut self) -> Hash {
        if self.hash == None {
            self.hash = Some(digest(self.encode_for_hash()));
        }
        self.hash.clone().unwrap()
    }

    pub fn sign(&mut self, private_key: &KeyPair) {
        let sig = private_key.sign(self.encode_for_hash());
        self.signature = Some(sig.signature.to_string());
        self.public_key = Some(private_key.public_key.to_string());
    }

    pub fn verify(&self) -> Result<(), String> {
        if self.signature == None || self.public_key == None {
            return Err("error: signature or public key missing".to_string());
        }
        let sig_result = new_sig_from_string(self.signature.clone().unwrap())?;
        let pk_result = new_pk_from_string(self.public_key.clone().unwrap())?;
        if sig_result.verify(&pk_result, self.encode_for_hash()) {
            return Ok(());
        }
        Err("error: invalid signature".to_string())
    }
}

pub fn decode_transaction(data: String) -> Result<Transaction, String> {
    //serde_json::from_str(&data).map_err(|_| "error decoding transaction".to_string());
    match serde_json::from_str(&data) {
        Ok(v) => Ok(v),
        Err(_) => Err("error decoding transaction".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_transaction() {
        let mut t = Transaction::new([0; 20], 5);
        println!("{}", t.hash());

        let mut data = t.encode();

        println!("{}", data.clone());
        let t_decode1 = decode_transaction(data.clone());
        assert_eq!(t_decode1.is_ok(), true);
        assert_eq!(t.hash() == t_decode1.unwrap().hash(), true);

        data += "a";
        let t_decode2 = decode_transaction(data);
        assert_eq!(t_decode2.is_err(), true);
    }

    #[test]
    fn test_sign() {
        let key_pair = KeyPair::new(0);
        let mut t = Transaction::new([0; 20], 5);
        println!("{}", t.hash());

        t.sign(&key_pair);
        assert_eq!(t.verify(), Ok(()));
    }
}
