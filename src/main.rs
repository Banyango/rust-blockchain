
extern crate crypto;
extern crate time;
extern crate iron;
extern crate router;
extern crate serde_json;
extern crate serde;
extern crate bodyparser;
extern crate pad;
#[macro_use] 
extern crate serde_derive;
#[macro_use] 
extern crate lazy_static;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::Mutex;
mod block;
mod miner;

lazy_static! {
    static ref BLOCKCHAIN: Mutex<Vec<block::Block>> = {
        Mutex::new(Vec::new())
    };    
}

static DIFFICULTY: i64 = 1;

fn push_block(block: block::Block) {
    BLOCKCHAIN.lock().unwrap().push(block);
}

fn do_for_blocks<F>(closure: F) where F : Fn(&block::Block) {
    for block in &*BLOCKCHAIN.lock().unwrap() {
        closure(block);
    }
}

fn main() {
        
    println!("Creating Genesis Block...");
    let genesis_block = block::Block {
        index:0,
        timestamp: time::get_time().sec.to_string(),
        bpm:0,
        hash:Some(String::from("")),
        prev_hash:String::from(""),
        difficulty:0,
        nonce:String::from(""),
    };

    println!("Server Started...");

    push_block(genesis_block);

    let mut router = Router::new();
    
    router.get("/", handler, "index");
    router.post("/", handle_write_block, "index");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let chain = &*BLOCKCHAIN.lock().unwrap();

    let output = serde_json::to_string(&chain);
    
    match output {
        Ok(expr) => return Ok(Response::with((status::Ok, expr))),
        Err(e) => return Err(iron::IronError::new(e, "Json parsing error" )),
    }
}

fn handle_write_block(req: &mut Request) -> IronResult<Response> {    
   let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => {
            println!("request recieved {}", json_body);
            match json_body.get("bpm") {
                Some(value) => {
                    let chain = &mut *BLOCKCHAIN.lock().unwrap();                    

                    match value.as_i64() {
                        Some(bpm) => {
                            println!("{}",value);
                            match miner::generate_block(&chain[chain.len()-1], bpm, DIFFICULTY) {
                                Ok(result) => {
                                    println!("Block was generated {}", result);
                                    
                                    if miner::is_block_valid(&result, &chain[chain.len()-1]) {
                                        println!("block is valid {}", result);

                                        let output = serde_json::to_string(&result);
                                        
                                        chain.push(result);      
                                        
                                        match output {
                                            Ok(expr) => return Ok(Response::with((status::Ok, expr))),
                                            Err(e) => return Err(iron::IronError::new(e, "Json parsing error" )),
                                        }                                                                        
                                    } else {
                                        println!("block was not valid {}", result);    
                                    }                                     
                                },
                                Err(_e) => return Ok(Response::with((status::InternalServerError, "Couldn't generate block")))
                            }
                        },
                        None => {
                            println!("No BPM");
                        }
                    }
                    
                    return Ok(Response::with((status::BadRequest, "")))     
                },
                None => return Ok(Response::with((status::BadRequest, ""))), 
            };
        },
        Ok(None) => return Ok(Response::with((status::BadRequest, ""))),
        Err(e) => return Err(iron::IronError::new(e, "Json parsing error" )),
    }

}