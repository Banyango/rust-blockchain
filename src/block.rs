use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index:i64,
    pub timestamp:String,
    pub bpm:i64,
    pub hash:Option<String>,
    pub prev_hash:String
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = serde_json::to_string(self);
        write!(f, "({})", result.unwrap())
    }
}

pub struct BlockChain {
    pub chain: Vec<Block>
}