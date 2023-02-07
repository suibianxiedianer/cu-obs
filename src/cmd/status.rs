use crate::Package;

#[derive(Debug)]
pub struct Status {
    repo: Option<String>,
    arch: Option<String>,
}

impl Status {
    pub fn new() -> Self {
        Status {
            repo: None,
            arch: None,
        }
    }

    pub fn set_repo(&mut self, repo: String) {
        self.repo = Some(repo);
    }

    pub fn set_arch(&mut self, arch: String) {
        self.arch = Some(arch);
    }

    pub fn apply(&self, pkg: &Package) -> crate::Result<()> {
        Ok(())
    }
}
