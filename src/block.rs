#![allow(unused)]

// for some reason, importing the module here is not working, so I 
// needed to use crate::linked_list
// mod linked_list;

use crate::linked_list::{LinkedList as List, Node};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockChain {
    pub blocks: List<Block>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub hash: String,
    id: String,
    transactions: List<Transaction>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxOut {
    pub_address: String,
    sats: u64,
}

impl Block {
    pub fn new(id: String, transactions: List<Transaction>) -> Self {
        Block {
            hash: String::new(),
            id,
            transactions
        }
    }

    pub fn compute_hash(&mut self){
        let serialized = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        self.hash = format!("{:x}", hasher.finalize())
    }
}