use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use mktemp::Temp;
use path_absolutize::*;

use crate::{Package, WORKSPACE};

#[derive(Debug, Clone)]
pub struct Workspace {
    root: PathBuf,
    temp: PathBuf,
}

impl Workspace {
    // init workspace, clean tempdir?
    // TODO: not confirm yet
    pub fn init<P: ?Sized + AsRef<OsStr>>(path: &P) -> crate::Result<Self> {
        let mut root = PathBuf::from(path).absolutize().unwrap().to_path_buf();
        let mut temp = root.join("_tmp");
        fs::create_dir_all(&temp)?;

        Ok(Workspace {
            root: root,
            temp: temp,
        })
    }

    /// root 字段
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 生成一个在 [工作区]/_tmp 下可用的空文件名
    pub fn temp_file(&self) -> PathBuf {
        Temp::new_file_in(&self.temp).unwrap().to_path_buf()
    }

    /// 在 [工作区]/_tmp 下新建并返回一个目录
    pub fn temp_dir(&self) -> PathBuf {
        let temp = Temp::new_dir_in(&self.temp).unwrap().to_path_buf();
        fs::create_dir(&temp).unwrap();
        temp
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
            fs::remove_dir_all(&path).map_err(|e| e.into())
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::{Package, Workspace};

    #[test]
    fn base() {
        let ws = Workspace::init("./workspace").unwrap();

        let _tmp_file = ws.temp_file();
        assert!(!_tmp_file.exists());

        let _tmp_dir = ws.temp_dir();
        fs::remove_dir(_tmp_dir).unwrap();

        let pkg = Package::new("prj", "pkg");

        // test env prepare
        let pkg_dir = ws.package_dir(&pkg);
        let mut _osc = pkg_dir.clone();
        _osc.push(".osc");

        fs::create_dir_all(&_osc.to_str().unwrap());

        use std::fs::File;
        let mut _demo = pkg_dir.clone();
        _demo.push("demo");
        {
            File::create(&_demo).unwrap();
        }

        // func test
        ws.clean_source(&pkg);
        assert!(!_demo.exists());
        assert!(_osc.exists());

        ws.remove_package(&pkg).unwrap();
        ws.remove_project(&pkg).unwrap();
        Workspace::remove(&ws.root);
    }
}
