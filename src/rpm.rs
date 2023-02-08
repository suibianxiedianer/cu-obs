use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use regex::Regex;

use crate::IsOk;

#[cfg(test)]
use crate::workspace::Workspace;

pub struct RPM {
    path: PathBuf,
    name: String,
    version: String,
    release: String,
}

/// RPM 工具
impl RPM {
    /// 新建一个 RPM，并执行初始化检查与字段填充
    pub fn new<P: ?Sized + AsRef<OsStr>>(path: &P) -> crate::Result<Self> {
        let path = PathBuf::from(path);
        path.canonicalize()?;
        let mut rpm = RPM {
            path,
            name: String::new(),
            version: String::new(),
            release: String::new(),
        };

        rpm.init()
    }

    /// 检查 rpm 的合法性，填充 name, version, release 信息
    fn init(mut self) -> crate::Result<Self> {
        if !self._is_rpm() {
            return Err(
                format!("{} is not a regular rpm file!", self.path.to_str().unwrap()).into(),
            );
        }
        self.name = self.get_name().unwrap();
        self.version = self.get_version().unwrap();
        self.release = self.get_release().unwrap();

        Ok(self)
    }

    /// 返回 RPM 包的名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 返回 RPM 包的版本
    pub fn version(&self) -> &str {
        &self.version
    }

    /// 返回 RPM 包的发布版本
    pub fn release(&self) -> &str {
        &self.release
    }

    /// 判断给定的路径文件是否为 rpm 格式
    pub fn is_rpm<P: ?Sized + AsRef<OsStr>>(path: &P) -> bool {
        // use file
        let _output = Command::new("file")
            .arg(path)
            .output()
            .expect("Failed to execute file.");

        let re = Regex::new(r"RPM v.*").unwrap();
        re.is_match(String::from_utf8(_output.stdout).unwrap().as_ref())
    }

    /// inner: is_rpm
    fn _is_rpm(&self) -> bool {
        Self::is_rpm(&self.path)
    }

    /// 判断给定的路径文件是否为 rpm 源码包
    pub fn is_src<P: ?Sized + AsRef<OsStr>>(path: &P) -> bool {
        let _output = Command::new("file")
            .arg(path)
            .output()
            .expect("Failed to execute file.");

        let re = Regex::new(r"RPM v.* src\n").unwrap();
        re.is_match(String::from_utf8(_output.stdout).unwrap().as_ref())
    }

    /// inner: is_src
    fn _is_src(&self) -> bool {
        Self::is_src(&self.path)
    }

    /// 获取软件包的 %{name} 字段
    pub fn get_name(&self) -> crate::Result<String> {
        self.query_format("%{name}")
    }

    /// 获取软件包的 %{version} 字段
    pub fn get_version(&self) -> crate::Result<String> {
        self.query_format("%{version}")
    }

    /// 获取软件包的 %{release} 字段
    pub fn get_release(&self) -> crate::Result<String> {
        self.query_format("%{release}")
    }

    /// 查询并格式化
    fn query_format(&self, qf: &str) -> crate::Result<String> {
        let _output = Command::new("rpm")
            .args(["-qp", "--qf", qf, self.path.to_str().unwrap()])
            .output()
            .expect("Failed to excute rpm.");

        _output.result()
    }

    /// 接收两个参数：<源码包路径> <Some(安装的路径)>
    /// 不指定安装目标路径，传 `None` 即可
    pub fn install_src(&self, target: Option<PathBuf>) -> crate::Result<()> {
        let mut args = vec![];
        let mut _specdir = String::new();
        let mut _sourcedir = String::new();

        if let Some(root) = target {
            let root = root.to_str().unwrap();
            _specdir = format!("_specdir {}", root);
            _sourcedir = format!("_sourcedir {}", root);
            args = vec!["--define", &_specdir, "--define", &_sourcedir];
        }

        let _output = Command::new("rpm")
            .arg("-i")
            .args(&args)
            .arg(&self.path.to_str().unwrap())
            .output()
            .expect("Failed to execute rpm.");

        _output.is_ok()
    }
}

#[cfg(test)]
mod test {
    use super::{RPM, Workspace};

    #[test]
    fn no_file() {
        match RPM::new("test/no-1.0-1.ule3.src.rpm") {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn not_rpm() {
        std::fs::File::create("test/empty").unwrap();
        match RPM::new("test/empty") {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn base() {
        let src_rpm = RPM::new("test/cc-1.0-1.ule3.src.rpm").unwrap();
        assert!(src_rpm._is_src());

        use std::fs;
        use std::path::PathBuf;
        use path_absolutize::*;
        let mut target = PathBuf::from("test/rpmbuild");
        fs::remove_dir_all(target.to_str().unwrap());
        fs::create_dir_all(target.to_str().unwrap()).unwrap();
        target = target.absolutize().unwrap().to_path_buf();

        let target = PathBuf::from(target);
        
        src_rpm.install_src(Some(target.clone())).unwrap();
    }
}
