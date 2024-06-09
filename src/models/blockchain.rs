use super::transaction::Transactions;
use super::block::{Block,cn_time};
use super::accounts::Account;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub struct Blockchain
{
    pub difficulty: usize,
    genesis_block: Block,
    pub chains: Vec<Block>,
    pub accounts: HashMap<String, Account>,
    pub pending_transactions: Vec<Transactions>,
}

impl Blockchain{
    pub fn new(difficulty: usize) -> Blockchain{
        let genesis_block = Block{
            index: 0,
            time: cn_time(),
            prev_hash: String::default(),
            nonce: 0,
            hash: String::default(),
            transactions: Vec::new(),
        };
        let mut chains = Vec::new();
        chains.push(genesis_block.clone());
        Blockchain{
            difficulty,
            genesis_block,
            chains,
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }
    

}
