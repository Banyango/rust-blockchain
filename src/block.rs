use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index:i64,
    pub timestamp:String,
    pub bpm:i64,
    pub hash:Option<String>,
    pub prev_hash:String,
    pub nonce:String,
    pub difficulty: i64,
}

impl Default for Block {
    fn default() -> Block {
        Block {
            index:0,
            timestamp:String::from(""),
            bpm:0,
            hash:None,
            prev_hash:String::from(""),
            nonce:String::from(""),
            difficulty:0,
        }
    }
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