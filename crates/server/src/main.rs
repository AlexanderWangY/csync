use core::panic;
use std::env;

use server::hash::compute_hashes_for_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected there to be 1 args (path)");
    }

    let path = args.get(1).unwrap();
    let hashes = compute_hashes_for_file(path).unwrap();

    for hash in &hashes {
        println!("{}", hash);
    }

    println!("Hashes for {}", path);
    println!("Total hashes: {}", hashes.len());
}
