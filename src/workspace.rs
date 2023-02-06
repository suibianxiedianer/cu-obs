use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Package, WORKSPACE};

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    temp: PathBuf,
}

impl Workspace {
    // init workspace, clean tempdir
    pub fn init() -> crate::Result<Self> {
        let root = PathBuf::from(WORKSPACE).canonicalize().unwrap();
        let temp = root.join("_tmp");
        fs::create_dir_all(&temp)?;
        Self::clean(&temp)?;

        Ok(Workspace {
            root,
            temp,
        })

    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn temp(&self) -> &PathBuf {
        &self.temp
    }

    pub fn project_dir(&self, package: &Package) -> PathBuf {
        self.root.join(&package.project)
    }

    pub fn package_dir(&self, package: &Package) -> PathBuf {
        self.project_dir(package).join(&package.package)
    }

    pub fn clean(path: &Path) -> crate::Result<()> {
        if path.is_file() {
            std::fs::remove_file(path)?
        } else {
            path.read_dir()?
                .for_each(|res| fs::remove_dir_all(res.unwrap().path()).unwrap());
        }

        Ok(())
    }
}
