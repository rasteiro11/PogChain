use crypto::crypto::SHA256;

use crate::crypto::crypto::Hasher;

mod block;
mod block_chain;
mod config;
mod crypto;

fn main() {
    // let hasher = SHA256::new();

    for i in 0..100 {
        println!("{}", SHA256::hash(i.to_string()));
    }
}
