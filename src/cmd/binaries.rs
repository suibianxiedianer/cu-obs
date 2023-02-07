use crate::{obs::OBS, workspace::Workspace, Package};

#[derive(Debug)]
pub struct Binaries {
    repo: String,
    arch: String,
    path: String,
}

impl Binaries {
    pub(crate) fn new(repo: impl ToString, arch: impl ToString, path: impl ToString) -> Self {
        Binaries {
            repo: repo.to_string(),
            arch: arch.to_string(),
            path: path.to_string(),
        }
    }

    pub(crate) fn apply(&self, pkg: &Package, ws: &Workspace) -> crate::Result<()> {
        Ok(())
    }
}
