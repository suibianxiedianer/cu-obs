use crate::Package;

#[derive(Debug)]
pub struct Status {
    repo: String,
    arch: String,
}

impl Status {
    pub(crate) fn new(repo: impl ToString, arch: impl ToString) -> Self {
        Status {
            repo: repo.to_string(),
            arch: arch.to_string(),
        }
    }

    pub(crate) fn apply(&self, pkg: &Package) -> crate::Result<()> {
        Ok(())
    }
}
