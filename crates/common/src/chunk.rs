pub struct Chunk {
    // Hash from Blake3
    pub hash: [u8; 32],

    // Data from disk
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_has_hash() {
        let chunk = Chunk {
            hash: [0; 32],
            data: Vec::new(),
        };
        assert_eq!(chunk.hash, [0; 32])
    }
}
