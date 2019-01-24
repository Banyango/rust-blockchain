use super::block::Block;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use pad::{PadStr, Alignment};

pub fn calculate_hash(block: &Block) -> String {
    // let record = self.index.to_string().push_str(&block.timestamp).push(self.bpm.to_string());
    let record = [block.index.to_string(), block.timestamp.clone(), block.bpm.to_string(), block.prev_hash.clone(), block.nonce.clone()].concat();

    let mut hasher = Sha256::new();

    hasher.input_str(&record);

    hasher.result_str()
}   

pub fn is_hash_valid(hash:&String, difficulty: i64) -> bool {
    let prefix = "".pad(1, '0', Alignment::Right, true);
    
    hash.starts_with(&prefix)
}

pub fn is_block_valid(new_block: &Block, old_block: &Block) -> bool {
    if old_block.index+1 != new_block.index {
        return false;
    }

    if old_block.hash.as_ref().expect("Hash is null").as_str() != new_block.prev_hash {
        return false;
    }

    let hash = calculate_hash(&new_block);

    if !new_block.hash.is_some() {
        return false;
    }

    if hash != new_block.hash.as_ref().unwrap().as_str() {
        println!("blocks dont match...");
        return false;
    }

    true
}

pub fn generate_block(old_block: &Block, bpm: i64, difficulty: i64) -> Result<Block, String> {
    println!("Generating Block...");
    let mut block = Block {
        index: old_block.index + 1,
        timestamp: time::get_time().sec.to_string(),
        bpm: bpm,   
        prev_hash:old_block.hash.clone().expect("Previous Hash is null"),
        hash:None,
        difficulty:difficulty,
        nonce:String::from(""),       
    };    

    let mut i = 0;
    
    loop {        
        let hex = format!("{:x}", i);

        block.nonce = hex;

        let hash = calculate_hash(&block);

        if !is_hash_valid(&hash, difficulty) {
            println!("{} do more work",hash);
            std::thread::sleep(std::time::Duration::from_secs(1));
        } else {
            println!("{} hash found",hash);
            block.hash = Some(hash);
            break;
        }

        i+=1;
    }    

    Ok(block)
}

pub fn should_replace_chain(new_blocks: &[Block], old_blocks: &[Block]) -> bool {
    if new_blocks.len() > old_blocks.len() {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {   
    use super::*; 
    use super::super::block::Block;
    use pad::{PadStr, Alignment};
    #[test]
    fn test_is_hash_valid() {
        assert_eq!(is_hash_valid(&String::from("0011"), 1), true);
    }

    #[test]
    fn test_prefix() {
        let prefix = "".pad(1, '0', Alignment::Right, true);
        assert_eq!(prefix, "0");
    }
    
    #[test]
    fn test_is_block_valid_index_wrong() {
        
        let old = Block {
            index:2,
             ..Default::default()
        };
        
        let new = Block {
            index:1,
             ..Default::default()
        };

        assert_eq!(is_block_valid(&new, &old), false);
    }

    #[test]
    fn test_is_block_valid_prev_hash_not_equal_to_hash() {
        
        let old = Block {
            index:0,
            hash:Some(String::from("1")),
             ..Default::default()
        };
        
        let new = Block {
            index:1,
            prev_hash:String::from("2"),
             ..Default::default()
        };

        assert_eq!(is_block_valid(&new, &old), false);
    }

    #[test]
    fn test_is_block_valid_new_hash_is_none() {
        
        let old = Block {
            index:0,
            hash:Some(String::from("1")),
             ..Default::default()
        };
        
        let new = Block {
            index:1,
            hash:None,
             ..Default::default()
        };

        assert_eq!(is_block_valid(&new, &old), false);
    }
}