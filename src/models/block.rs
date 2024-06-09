use super::transaction::Transactions;
use super::blockchain::Blockchain;
use chrono::prelude::*;
use serde::{Serialize,Deserialize};
use sha2::{Sha256,Digest};
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block{
    pub index: u64,
    pub time: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub transactions: Vec<Transactions>,
}

impl Block{
    pub fn new(index: u64,prev_hash: String) -> Block{
        Block{
            index,
            time:cn_time(),
            prev_hash,
            nonce: 0,
            hash: String::default(),
            transactions: Vec::default(),
        }
    }
    //导入块数据计算哈希
    pub fn cul_hash(&mut self) -> String{
        let  block_data = self.clone();
        let mut hasher = Sha256::new();
        let ser_block_data = serde_json::to_string(&block_data).expect("序列化块数据失败");
        hasher.update(ser_block_data);
        format!("{:x}",hasher.finalize())
    }
    //挖矿找到正确的nonce
    pub fn mine(&mut self,blockc: &Blockchain){
        loop{
            if !self.hash.starts_with(&"0".repeat(blockc.difficulty)){
                self.nonce += 1;
                self.hash = self.cul_hash();
            }
            else{
                break;
            }
        }
    }
}
pub fn cn_time() -> String {
    let offset = FixedOffset::east_opt(8 * 3600);
    let utc_time = Utc::now();
    if let Some(offset) = offset{
        let local_time = utc_time.with_timezone(&offset);
        return local_time.format("%Y-%m-%d %H:%M:%S").to_string();
    };
    String::from("获取时间失败")
}