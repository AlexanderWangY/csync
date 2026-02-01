use core::panic;
use std::env::consts::OS;

// The main files are stored in ~/.csync
// Manifests are stored in ~/.csync/manifests/
// Data is stored in ~/.csync/data/
// Configs are stored in ~/.csync/config.toml
pub fn init_filesystem() {
    // Server MUST be Linux based. For now...
    if OS != "linux" {
        panic!("Your operatin system is not Linux. Aborting. Switch to linux fool.")
    }
}
