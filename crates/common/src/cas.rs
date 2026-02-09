use std::{fs, sync::Arc};

use crate::storage::{Storage, StorageError};

#[derive(Debug)]
pub enum CasError {
    StorageErr(StorageError),
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

    // Check if object exists
    pub fn exists(&self, hash: &[u8]) -> bool {
        let objects_dir = self.storage.objects_dir();

        let shard1 = format!("{:02x}", hash[0]);
        let shard2 = format!("{:02x}", hash[1]);
        let object_name = hex::encode(hash);

        let shard_path = objects_dir.join(shard1).join(shard2).join(object_name);

        shard_path.exists()
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

    pub fn get_object(&self, hash: &[u8; 32]) {
        if !self.exists(hash) {
            // Should return error here
        }
    }
}
