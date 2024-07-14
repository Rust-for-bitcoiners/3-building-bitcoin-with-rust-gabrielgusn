#![allow(unused)]

// for some reason, importing the module here is not working, so I 
// needed to use crate::linked_list
// mod linked_list;

use crate::linked_list::{LinkedList as List, Node};

struct BlockChain {
    blocks: List<Block>
}

struct Block {
    hash: String,
    id: u128,
    transactions: List<Transaction>
}

struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String
}

struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String
}

struct TxOut {
    pub_address: String,
    sats: u64,
}