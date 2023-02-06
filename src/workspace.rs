use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Package, WORKSPACE};

#[derive(Debug)]
pub struct Workspace {
    path: PathBuf,
    temp: PathBuf,
}

impl Workspace {
    // init workspace, clean tempdir
    pub(crate) fn init() -> crate::Result<Self> {
        let path = PathBuf::from(WORKSPACE).canonicalize().unwrap();
        let temp = path.join("_tmp");
        fs::create_dir_all(&temp)?;
        Self::clean(&temp)?;

        Ok(Workspace {
            path,
            temp,
        })

    }

    pub(crate) fn temp(&self) -> &PathBuf {
        &self.temp
    }

    pub(crate) fn project_dir(&self, package: &Package) -> PathBuf {
        self.path.join(&package.project)
    }

    pub(crate) fn package_dir(&self, package: &Package) -> PathBuf {
        self.project_dir(package).join(&package.package)
    }

    pub(crate) fn clean(path: &Path) -> crate::Result<()> {
        if path.is_file() {
            std::fs::remove_file(path)?
        } else {
            path.read_dir()?
                .for_each(|res| fs::remove_dir_all(res.unwrap().path()).unwrap());
        }

        Ok(())
    }
}
