use std::{
    env::{consts::OS, home_dir},
    fs::{File, create_dir_all},
    path::PathBuf,
};

// The main files are stored in ~/.csync
// Manifests are stored in ~/.csync/manifests/
// Data is stored in ~/.csync/data/
// Configs are stored in ~/.csync/config.toml
pub fn init_filesystem() -> Result<PathBuf, String> {
    // Server MUST be Linux based. For now...
    if OS != "linux" {
        return Err("csync currently only supports Linux operating systems".to_string());
    }

    let home_dir = home_dir().unwrap();
    println!("Your home dir {:?}", home_dir);

    // Create ~/.csync and all children if not exists
    let base_dir = home_dir.join(".csync");
    let data_dir = base_dir.join("data");
    let manifest_dir = base_dir.join("manifests");
    let config_path = base_dir.join("config.toml");

    create_dir_all(data_dir).map_err(|e| e.to_string())?;
    create_dir_all(manifest_dir).map_err(|e| e.to_string())?;

    if !config_path.exists() {
        File::create(&config_path).map_err(|e| e.to_string())?;
    }

    Ok(base_dir.to_path_buf())
}
