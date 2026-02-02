use std::{
    fs::File,
    io::{BufReader, Error, Read},
    path::Path,
};

// 1 MB
const CHUNK_SIZE: usize = 1024 * 1024;

pub fn hash_bytes(b: &[u8]) -> [u8; 32] {
    let hash = blake3::hash(b);

    hash.into()
}

pub fn chunk_and_hash_file(path: impl AsRef<Path>) -> Result<Vec<[u8; 32]>, Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; CHUNK_SIZE];

    let mut hashes: Vec<[u8; 32]> = Vec::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hashes.push(hash_bytes(&buffer[..bytes_read]));
    }

    Ok(hashes)
}
