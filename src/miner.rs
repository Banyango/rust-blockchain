use super::block::Block;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn calculate_hash(block: &Block) -> String {
    // let record = self.index.to_string().push_str(&block.timestamp).push(self.bpm.to_string());
    let record = [block.index.to_string(), block.timestamp.clone(), block.bpm.to_string(), block.prev_hash.clone()].concat();

    let mut hasher = Sha256::new();

    hasher.input_str(&record);

    hasher.result_str()
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

pub fn generate_block(old_block: &Block, bpm: i64) -> Result<Block, String> {
    println!("Generating Block...");
    let mut block = Block {
        index: old_block.index + 1,
        timestamp: time::get_time().sec.to_string(),
        bpm: bpm,   
        prev_hash:old_block.hash.clone().expect("Previous Hash is null"),
        hash:None        
    };    

    block.hash = Some(calculate_hash(&block));

    Ok(block)
}

pub fn should_replace_chain(new_blocks: &[Block], old_blocks: &[Block]) -> bool {
    if new_blocks.len() > old_blocks.len() {
        return true;
    }
    false
}