use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn hash_chunk(b: &[u8]) -> String {
    let hash = blake3::hash(b);

    hash.to_string()
}

pub fn compute_hashes_for_file(
    path: impl AsRef<Path>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 4096];

    let mut result = Vec::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        result.push(hash_chunk(&buffer[..bytes_read]));
    }

    Ok(result)
}
