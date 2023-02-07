use std::fs;
use std::path::{Path, PathBuf};

use crate::{Package, WORKSPACE};

#[derive(Debug, Clone)]
pub struct Workspace {
    root: PathBuf,
    temp: PathBuf,
}

impl Workspace {
    // init workspace, clean tempdir?
    // TODO: not confirm yet
    pub fn init() -> crate::Result<Self> {
        let root = PathBuf::from(WORKSPACE).canonicalize().unwrap();
        let temp = root.join("_tmp");
        fs::create_dir_all(&temp)?;

        Ok(Workspace { root, temp })
    }

    /// root 字段
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// temp 字段, temp 还没想好怎么用
    pub fn temp(&self) -> &PathBuf {
        &self.temp
    }

    /// 工作区中对应项目的路径
    pub fn project_dir(&self, pkg: &Package) -> PathBuf {
        self.root.join(&pkg.project())
    }

    /// 工作区中对应包的路径
    pub fn package_dir(&self, pkg: &Package) -> PathBuf {
        self.project_dir(pkg).join(&pkg.package())
    }

    /// 删除项目的所在路径
    pub fn remove_project(&self, pkg: &Package) -> crate::Result<()> {
        Self::remove(&self.project_dir(pkg))
    }

    /// 删除包的所在路径
    pub fn remove_package(&self, pkg: &Package) -> crate::Result<()> {
        Self::remove(&self.package_dir(pkg))
    }

    /// 对于指定的包，仅删除其源码文件，而保留 `.osc` 信息
    pub fn clean_source(&self, pkg: &Package) -> crate::Result<()> {
        self.package_dir(pkg).read_dir()?.for_each(|entry| {
            let entry = entry.unwrap();
            if entry.file_name() == ".osc" {
                return;
            }
            Self::remove(&entry.path()).unwrap();
        });

        Ok(())
    }

    /// rm -rf <Path>
    pub fn remove(path: &Path) -> crate::Result<()> {
        if path.is_file() {
            fs::remove_file(path).map_err(|e| e.into())
        } else {
            fs::remove_dir_all(&path)?;
            fs::remove_dir(path).map_err(|e| e.into())
        }
    }
}
