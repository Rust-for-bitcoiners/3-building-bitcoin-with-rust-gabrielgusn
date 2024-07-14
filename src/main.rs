mod linked_list;
mod block;
mod mresult;

use block::{Block, BlockChain};
use linked_list::LinkedList as List;

fn main() {
    // Example usage
    let mut blockchain = BlockChain {
        blocks: List::new(),
    };

    let transactions = List::new(); // You'd populate this with actual transactions
    let mut first_block = Block::new(String::new(), transactions);
    first_block.compute_hash();
    blockchain.blocks.push(first_block.clone());

    let remaining_transactions = List::new();
    let mut second_block = Block::new(first_block.hash.clone(), remaining_transactions);
    second_block.compute_hash();

    blockchain.blocks.push(second_block);

    // Print the blockchain
    let serialized = serde_json::to_string(&blockchain).unwrap();
    println!("{}", serialized);
}