use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Transactions{
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signture: Vec<u8>,
    pub public_key: Vec<u8>,
    pub time: String,
    pub service_fee: u64,
}
impl Transactions{
    pub fn new(sender: String,receiver: String,amount: u64,signture:Vec<u8>,public_key: Vec<u8>,time: String,service_fee: u64) -> Transactions{
        Transactions{
            sender,
            receiver,
            amount,
            signture,
            public_key,
            time,
            service_fee,
        }
    }
}