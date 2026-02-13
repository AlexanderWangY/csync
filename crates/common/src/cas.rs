use std::{
    fs,
    path::{self, Path, PathBuf},
    sync::Arc,
};

use crate::{
    chunk::Chunk,
    storage::{Storage, StorageError},
};

#[derive(Debug)]
pub enum CasError {
    StorageErr(StorageError),
    ObjectNotFound,
    Unknown,
}

impl From<StorageError> for CasError {
    fn from(value: StorageError) -> Self {
        Self::StorageErr(value)
    }
}

pub struct Cas {
    pub storage: Arc<Storage>,
}

impl Default for Cas {
    fn default() -> Self {
        Self {
            storage: Arc::new(Storage::new().unwrap()),
        }
    }
}

impl Cas {
    pub fn new() -> Self {
        Cas::default()
    }

    pub fn new_with_storage(storage: Arc<Storage>) -> Self {
        Self { storage }
    }

    fn construct_hash_path(&self, hash: &[u8; 32]) -> PathBuf {
        let objects_dir = self.storage.objects_dir();

        let shard1 = format!("{:02x}", hash[0]);
        let shard2 = format!("{:02x}", hash[1]);
        let object_name = hex::encode(hash);

        objects_dir.join(shard1).join(shard2).join(object_name)
    }

    // Check if object exists
    pub fn object_exists(&self, hash: &[u8; 32]) -> bool {
        self.construct_hash_path(hash).exists()
    }

    pub fn put_object(&self, hash: &[u8; 32], data: &[u8]) -> Result<(), CasError> {
        // Crawl through 2 layer data layer
        // 1) Layer 1 doesnt exist, please create
        // 2) Layer 2 doent exist, please create
        // 3) Use storage.atomic_write() to write into a file with name = hash

        let objects_dir = self.storage.objects_dir();

        // Formats first 2 bytes of hash to hex representation
        let shard1 = format!("{:02x}", hash[0]);
        let shard2 = format!("{:02x}", hash[1]);
        let object_name = hex::encode(hash);

        // Build object store shard path
        let shard_dir = objects_dir.join(shard1).join(shard2);
        let shard_object_path = shard_dir.join(object_name);

        // In CAS, don't create the file if it already exists.
        if shard_object_path.exists() {
            return Ok(());
        }

        self.storage.atomic_write(&shard_object_path, data)?;

        Ok(())
    }

    pub fn get_object(&self, hash: &[u8; 32]) -> Result<Chunk, CasError> {
        if !self.object_exists(hash) {
            return Err(CasError::ObjectNotFound);
        }

        let path = self.construct_hash_path(hash);

        let bytes = self
            .storage
            .read_blob(&path)
            .map_err(CasError::StorageErr)?;

        Ok(Chunk {
            hash: *hash,
            data: bytes,
        })
    }
}
