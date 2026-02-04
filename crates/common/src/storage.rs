use std::{
    env::{consts::OS, home_dir},
    fs::{File, create_dir_all},
    io::{BufReader, Error, Read},
    path::{Path, PathBuf},
};

use crate::hash::hash_bytes;

pub enum StorageError {
    UnsupportedOS,
    NoHomeDir,
    PathOutsideRoot,
    InvalidFileLocation,
}

pub struct Storage {
    pub root: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self, StorageError> {
        if OS != "linux" {
            return Err(StorageError::UnsupportedOS);
        }

        let home_dir = home_dir().ok_or(StorageError::NoHomeDir)?;
        let root = home_dir.join(".csync");

        let data = root.join("data");
        let manifests = root.join("manifests");
        let config = root.join("config.toml");

        create_dir_all(&data).unwrap();
        create_dir_all(&manifests).unwrap();

        if !config.exists() {
            File::create(&config).unwrap();
        }

        Ok(Self { root })
    }

    pub fn data_dir(&self) -> PathBuf {
        self.root.join("data")
    }

    pub fn manifest_dir(&self) -> PathBuf {
        self.root.join("manifests")
    }

    pub fn config_path(&self) -> PathBuf {
        self.root.join("config.toml")
    }

    pub fn atomic_write(&self, path: &Path, bytes: &[u8]) -> Result<(), StorageError> {
        // Firstly, ensure the path is a child of the root to remove
        // external write invariant

        let is_child_path = path.starts_with(&self.root);
        if !is_child_path {
            return Err(StorageError::PathOutsideRoot);
        }

        let filename = path.file_name().ok_or(StorageError::InvalidFileLocation)?;
        let path_root = path.parent().ok_or(StorageError::InvalidFileLocation)?;
        // Then, create a new file called the filename + .temp
        let mut temp_file = PathBuf::from(path_root);
        temp_file.push(filename);
        temp_file.set_extension(".temp");

        // Write to the temp file in the same dir
        // Then renaming the file to the write file is atomic
        // Done!

        Ok(())
    }
}
