use crate::{obs::OBS, workspace::Workspace, Package};

#[derive(Debug)]
pub struct Binaries {
    repo: String,
    arch: String,
    path: Option<String>,
}

impl Binaries {
    pub fn new(repo: impl ToString, arch: impl ToString) -> Self {
        Binaries {
            repo: repo.to_string(),
            arch: arch.to_string(),
            path: None,
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn apply(&self, pkg: &Package, ws: &Workspace) -> crate::Result<()> {
        Ok(())
    }
}
