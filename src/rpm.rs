use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::IsOk;

pub struct RPM;

/// RPM 工具
impl RPM {
    /// 判断给定的路径文件是否为 rpm 源码包
    /// 预留，暂无用处
    pub fn _is_src(file: &Path) -> crate::Result<()> {
        // use file
        let _type = Command::new("file")
            .arg(file.to_str().unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute file.");

        // grep key words
        let _output = Command::new("grep")
            .arg("RPM v.* src$")
            .stdin(Stdio::from(_type.stdout.unwrap()))
            .output()
            .expect("failed to execute grep");

        _output.is_ok()
    }

    /// 获取软件包的 %{name} 字段
    pub fn get_name(file: &Path) -> crate::Result<String> {
        let _output = Command::new("rpm")
            .args(["-qp", "--qf", "%{name}", file.to_str().unwrap()])
            .output()
            .expect("Failed to excute rpm.");

        _output.result()
    }

    /// 获取软件包的 %{version} 字段
    pub fn get_version(file: &Path) -> crate::Result<String> {
        let _output = Command::new("rpm")
            .args(["-qp", "--qf", "%{version}", file.to_str().unwrap()])
            .output()
            .expect("Failed to excute rpm.");

        _output.result()
    }

    /// 获取软件包的 %{release} 字段
    pub fn get_release(file: &Path) -> crate::Result<String> {
        let _output = Command::new("rpm")
            .args(["-qp", "--qf", "%{release}", file.to_str().unwrap()])
            .output()
            .expect("Failed to excute rpm.");

        _output.result()
    }

    /// 接收两个参数：<源码包路径> <Some(安装的路径)>
    /// 不指定安装目标路径，传 `None` 即可
    pub fn install_src(file: &Path, target: Option<PathBuf>) -> crate::Result<()> {
        let mut args = vec![];
        let mut _specdir = String::new();
        let mut _sourcedir = String::new();

        if let Some(root) = target {
            let root = root.to_str().unwrap();
            _specdir = format!("_spcedir {}", root);
            _sourcedir = format!("_sourcedir {}", root);
            args = vec!["--define", &_specdir, "--define", &_sourcedir];
        }

        let _output = Command::new("rpm")
            .arg("-i")
            .args(&args)
            .arg(file.to_str().unwrap())
            .output()
            .expect("Failed to execute rpm.");

        _output.is_ok()
    }
}
