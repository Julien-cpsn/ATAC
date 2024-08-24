use std::path::PathBuf;
use directories::UserDirs;

pub fn expand_tilde(path_buf: PathBuf) -> PathBuf {
    if !path_buf.starts_with("~/") {
        return path_buf;
    }

    match UserDirs::new() {
        Some(user_dirs) => {
            let mut home_dir = user_dirs.home_dir().to_path_buf();
            home_dir.push(path_buf.strip_prefix("~/").unwrap());
            home_dir
        },
        None => panic!("No home directory found when trying to expand \"~\"")
    }
}