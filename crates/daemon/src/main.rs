use core::panic;
use std::env;

use common::{hash::chunk_and_hash_file, storage::init_filesystem};

fn main() {
    if let Err(s) = init_filesystem() {
        panic!("{s}");
    }
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected there to be 1 args (path)");
    }

    let path = args.get(1).unwrap();
    let hashes = chunk_and_hash_file(path).unwrap();

    for hash in &hashes {
        println!("{:x?}", hash);
    }

    println!("Hashes for {}", path);
    println!("Total hashes: {}", hashes.len());
}
