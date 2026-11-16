use std::{
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

/// Returns the base directory for user-specific state.
///
/// See https://specifications.freedesktop.org/basedir/0.8/.
pub fn xdg_state_home(home_dir: &Path) -> PathBuf {
    env::var_os("XDG_STATE_HOME")
        .and_then(|value| {
            if !value.is_empty() {
                Some(PathBuf::from(value))
            } else {
                None
            }
        })
        .unwrap_or_else(|| home_dir.join(".local/state"))
}

pub fn ensure_parent_exists(path: &Path) -> Result<(), std::io::Error> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    match fs::create_dir_all(parent) {
        Err(err) if err.kind() == ErrorKind::AlreadyExists => Ok(()),
        res => res,
    }
}
