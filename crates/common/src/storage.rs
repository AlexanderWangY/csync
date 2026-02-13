use std::{
    env::{consts::OS, home_dir},
    error,
    fmt::{self, Display},
    fs::{self, File, remove_file, rename},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum StorageError {
    UnsupportedOS,
    NoHomeDir,
    PathOutsideRoot,
    InvalidFileLocation,
    FileDoesNotExist,
    IoError(io::Error),
}

impl Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOS => write!(f, "Only Linux is supported"),
            Self::NoHomeDir => write!(f, "Could not find home directory"),
            Self::PathOutsideRoot => write!(f, "Attempted to write outside of storage root"),
            Self::InvalidFileLocation => write!(f, "Invalid file path or filename"),
            Self::FileDoesNotExist => write!(f, "File does not exist"),
            Self::IoError(e) => write!(f, "I/O Error: {}", e),
        }
    }
}

impl error::Error for StorageError {}

/// A local file-system storage abstraction for `csync`
///
/// `Storage` manages the directory hierarchy for data objects/blobs, manifests, and
/// configuration, enforcing path safety and providing atomic write operations
/// within the `~/.csync` root.
#[derive(Debug)]
pub struct Storage {
    pub root: PathBuf,
}

impl AsRef<Path> for Storage {
    fn as_ref(&self) -> &Path {
        &self.root
    }
}

impl Storage {
    /// Returns a new `Storage` struct whose root is the `/home/$USER/.csync` directory.
    ///
    /// # Errors
    /// Throws `StorageError::NoHomeDir` if current user's home directory is not available.
    /// Also throws `StorageError::IoError(io::Error)` if it fails to create `objects/`, `manifests/`
    /// and/or `config.toml`.
    pub fn new() -> Result<Self, StorageError> {
        if OS != "linux" {
            return Err(StorageError::UnsupportedOS);
        }

        let home_dir = home_dir().ok_or(StorageError::NoHomeDir)?;
        Self::with_root(home_dir.join(".csync"))
    }

    pub fn with_root(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let root = path.as_ref().to_path_buf();

        let objects = root.join("objects");
        let manifests = root.join("manifests");
        let config = root.join("config.toml");

        fs::create_dir_all(&objects).map_err(StorageError::IoError)?;
        fs::create_dir_all(&manifests).map_err(StorageError::IoError)?;

        if !config.exists() {
            File::create(&config).map_err(StorageError::IoError)?;
        }

        Ok(Self { root })
    }

    pub fn objects_dir(&self) -> PathBuf {
        self.root.join("objects")
    }

    pub fn manifests_dir(&self) -> PathBuf {
        self.root.join("manifests")
    }

    pub fn config_path(&self) -> PathBuf {
        self.root.join("config.toml")
    }

    /// Checks whether a given path is within root
    /// Useful for ensuring operations stay within the set root
    fn within_root(&self, path: &Path) -> Result<bool, StorageError> {
        let root = self.root.canonicalize().map_err(StorageError::IoError)?;
        let candidate = path.canonicalize().map_err(StorageError::IoError)?;

        Ok(candidate.starts_with(root))
    }

    pub fn read_blob(&self, path: &Path) -> Result<Vec<u8>, StorageError> {
        // This function does the following
        // 1) Ensures path is within the root
        // 2) Ensure file exists
        // 3) Returns file content in bytes

        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(StorageError::InvalidFileLocation);
        }

        if !self.within_root(path)? {
            return Err(StorageError::InvalidFileLocation);
        }

        if !path.exists() || path.is_dir() {
            return Err(StorageError::FileDoesNotExist);
        }

        let bytes: Vec<u8> = fs::read(path).map_err(StorageError::IoError)?;

        Ok(bytes)
    }

    /// Atomically writes `bytes` to the specified `path`.
    ///
    /// This function ensures that the file is either fully written or not written at all,
    /// even in the event of a system crash. It achieves this by:
    /// 1. Writing to a unique temporary file in the same directory.
    /// 2. Synchronizing data to physical storage (`fsync`).
    /// 3. Atomically renaming the temporary file to the destination.
    ///
    /// # Errors
    /// Returns `StorageError::PathOutsideRoot` if the path escapes the storage root,
    /// or `StorageError::IoError` if disk synchronization or renaming fails.
    pub fn atomic_write(&self, path: &Path, bytes: &[u8]) -> Result<(), StorageError> {
        // Ensure no '..' in path.
        // Example: "hello/../what" X - not allowed
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(StorageError::InvalidFileLocation);
        }

        // Prevent outside of root invariant
        if !path.starts_with(&self.root) {
            return Err(StorageError::PathOutsideRoot);
        }

        let filename = path.file_name().ok_or(StorageError::InvalidFileLocation)?;
        let parent_dir = path.parent().ok_or(StorageError::InvalidFileLocation)?;
        fs::create_dir_all(parent_dir).map_err(StorageError::IoError)?;

        // Generate random suffix for temp file to prevent collisions
        let random_suffix = fastrand::u64(..);
        let mut temp_filename = filename.to_os_string();
        temp_filename.push(format!(".{}.temp", random_suffix));

        let temp_path = parent_dir.join(temp_filename);

        // Create and write to temp file
        {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp_path)
                .map_err(StorageError::IoError)?;

            file.write_all(bytes).map_err(StorageError::IoError)?;

            file.sync_all().map_err(StorageError::IoError)?;
        }

        if let Err(e) = rename(&temp_path, path) {
            // Remove temporary file if failed to rename
            let _ = remove_file(&temp_path);
            return Err(StorageError::IoError(e));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn create_basic() {
        let temp = tempdir().unwrap();
        let storage = Storage::with_root(temp.path().join(".csync")).unwrap();

        let expected_root = temp.path().join(".csync");
        let expected_objects = expected_root.join("objects");
        let expected_manifests = expected_root.join("manifests");
        let expected_config = expected_root.join("config.toml");
        assert_eq!(expected_root, storage.root);
        assert_eq!(expected_objects, storage.objects_dir());
        assert_eq!(expected_manifests, storage.manifests_dir());
        assert_eq!(expected_config, storage.config_path());
    }

    #[test]
    fn write_bytes() {
        let temp = tempdir().unwrap();
        let storage = Storage::with_root(temp.path().join(".csync")).unwrap();

        let test_string = "Hello, world!";

        let mut file_path = storage.objects_dir();
        file_path.push("test.txt");

        storage
            .atomic_write(&file_path, test_string.as_bytes())
            .unwrap();

        // Read from file and compare
        let bytes: Vec<u8> = fs::read(file_path).unwrap();
        assert_eq!(test_string, String::from_utf8(bytes).unwrap());
    }

    #[test]
    fn invalid_path() {
        let temp = tempdir().unwrap();
        let storage = Storage::with_root(temp.path().join(".csync")).unwrap();

        let bad_path = storage.root.join("objects/../bad.txt");

        let err = storage.atomic_write(&bad_path, b"Randomstuff").unwrap_err();

        assert!(matches!(err, StorageError::InvalidFileLocation));
    }
}
