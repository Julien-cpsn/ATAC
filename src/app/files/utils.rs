use std::path::PathBuf;

pub(crate) fn expand_tilde(path_buf: PathBuf) -> PathBuf {
    if !path_buf.starts_with("~/") {
        return path_buf;
    }

    match dirs::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(path_buf.strip_prefix("~/").unwrap());
            return home_dir;
        },
        None => panic!("No home directory found when trying to expand ~")
    }
}