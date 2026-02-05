use std::sync::Arc;

use crate::storage::Storage;

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
}
