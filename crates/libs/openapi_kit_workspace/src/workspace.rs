use crate::error::Error;
use std::path::{Path, PathBuf};

pub struct Workspace {
    pub path: PathBuf,
}

impl Workspace {
    pub fn load() -> Result<Self, Error> {
        let current_dir = std::env::current_dir()?;
        Self::load_recursive(current_dir)
    }

    /// Recursively look for a ".openapi" directory to mark the root directory of the workspace.
    ///
    fn load_recursive<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let full_path = path.join(".openapi");

        if !full_path.exists() {
            let Some(parent) = path.parent() else {
                return Err(Error::NoWorkspaceFound);
            };

            Self::load_recursive(parent)
        } else {
            Ok(Workspace {
                path: path.to_path_buf(),
            })
        }
    }
}
