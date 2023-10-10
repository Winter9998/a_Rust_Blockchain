extern crate sha2;
use sha2::{Sha256, Digest};
use std::rand::Rng;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};



pub struct Block {
    pub block_number: i32,
    pub nonce: i32, 
    pub timestamp: i32,
    pub hash: String,
    pub prev_hash: String,

}

pub struct Blockchain {
    pub chain: Vec<Block>,
}


pub fn random_word(number: i32) -> String {
    if (number < 0 || number >= 5){
        panic!("Not possible");

    }
    
    let mut word: [&str; 5] = ["Fries","Time","Reality","Rich","Work"];
    return word[number];
}



impl Block {
    pub fn newBlock(block_number: i32,nonce: i32, timestamp: i32, hash: String,prev_hash: String,) -> Block {
        
        let mut blocknum: i32 = 0; // block number
        block_number = blocknum + 1;
        
        let mut nonceNum = HashSet::new();
        
        let mut rng = rand::thread_rng();
        nonce = rng.gen_range(999..99999);  // nonce part of the block 

        if !nonceNum.contains(nonce) {
            let mut new_Nonce = rand::thread_rng().gen_range(999..99999);
        } else {
            !nonceNum.insert(nonce);
        }

        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Error while fetching the block timestamp");
        let timestamp = since_the_epoch.as_secs() as i64;

        prev_hash = match ZPHBlockchain.last() {
            Some(value) => value.to_string(),
            None => println!("The vector is empty"),
        };
         
        genhash = Sha256::new(); // hashing part of the block
        genhash.update(format!("{}{}{}", block_number, nonce, timestamp, prev_hash));
        hash = genhash.finalize();



        Block {
            block_number,
            nonce,
            timestamp,
            hash,
            prev_hash
        }

        
}

}
fn main() {
    let ZPHBlockchain = Blockchain {chain: Vec::new()};

}