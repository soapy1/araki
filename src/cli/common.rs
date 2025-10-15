use directories::UserDirs;
use std::path::PathBuf;
use std::fs;

const AKARI_ENVS_DIR: &str = "akari/envs";

/// Get the user's akari envs directory, which by default
/// is placed in their home directory
pub fn get_default_akari_envs_dir() -> Option<PathBuf> {
    let Some(akari_envs_dir) = UserDirs::new()
        .map(|dirs| dirs.home_dir().join(AKARI_ENVS_DIR))
    else {
        return UserDirs::new()
        .map(|dirs| dirs.home_dir().join(AKARI_ENVS_DIR))
    };

    if !akari_envs_dir.exists() {
        println!("akari envs dir does not exist. Creating it at {:?}", akari_envs_dir);
        let _ = fs::create_dir_all(akari_envs_dir);
    }

    UserDirs::new()
        .map(|dirs| dirs.home_dir().join(AKARI_ENVS_DIR))
}

