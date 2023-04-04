use crate::types::hash::Hash;
use secp256k1::All;
use secp256k1::{
    rand::{rngs, SeedableRng},
    Message, PublicKey, Secp256k1, SecretKey, Signature,
};
use serde::{Deserialize, Serialize};
use sha256::{digest, try_digest};
use std::str::FromStr;

pub struct KeyPair {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    secp: Secp256k1<All>,
}

pub fn new_pk_from_string(public_key: String) -> Result<PublicKey, String> {
    match PublicKey::from_str(&public_key) {
        Ok(v) => Ok(v),
        Err(_) => Err("error: invalid public key given".to_string()),
    }
}

fn string_to_message(data: String) -> Message {
    let binding = digest(data);
    let msg_hash = binding.as_bytes();
    let mut msg_bytes: [u8; 32] = [0; 32];
    for i in 0..32 {
        msg_bytes[i] = msg_hash[i * 2] + msg_hash[i * 2 + 1];
    }
    Message::from_slice(&msg_bytes).unwrap()
}

impl KeyPair {
    pub fn new(seed: u64) -> Self {
        let secp = Secp256k1::new();
        let mut rng = rngs::StdRng::seed_from_u64(seed);
        let r = secp.generate_keypair(&mut rng);
        Self {
            private_key: r.0,
            public_key: r.1,
            secp,
        }
    }

    pub fn sign(&self, data: String) -> Sig {
        //let r2 = secp.generate_keypair(&mut rng);
        let signature = self.secp.sign(&string_to_message(data), &self.private_key);
        new_sig(signature)
    }
}

#[derive(Debug)]
pub struct Sig {
    pub signature: Signature,
    secp: Secp256k1<All>,
}

pub fn new_sig_from_string(signature: String) -> Result<Sig, String> {
    match Signature::from_str(&signature) {
        Ok(v) => Ok(new_sig(v)),
        Err(_) => Err("error: invalid signature given".to_string()),
    }
}

pub fn new_sig(signature: Signature) -> Sig {
    Sig {
        signature,
        secp: Secp256k1::new(),
    }
}

impl Sig {
    pub fn verify(&self, public_key: &PublicKey, data: String) -> bool {
        let msg = string_to_message(data);
        let result = self.secp.verify(&msg, &self.signature, public_key);
        result.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_success() {
        let keypair = KeyPair::new(0);
        let msg = "hello".to_string();

        let sig = keypair.sign(msg.clone());
        assert_eq!(sig.verify(&keypair.public_key, msg), true);
    }

    #[test]
    fn test_sign_verify_failure() {
        let keypair = KeyPair::new(1);
        println!("{:?}", keypair.private_key.to_string());
        let msg = "hello".to_string();

        let sig = keypair.sign(msg.clone());
        assert_eq!(sig.verify(&keypair.public_key, "hi".to_string()), false);

        let other_keypair = KeyPair::new(2);
        println!("{:?}", other_keypair.private_key.to_string());
        assert_eq!(
            sig.verify(&other_keypair.public_key, "hello".to_string()),
            false
        );
    }

    #[test]
    fn test_string_to_message() {
        let s = digest("hello");
        let m = string_to_message(s.clone());
        //assert_eq!(s, s2);
    }
}
