use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    Package,
    workspace::Workspace,
};

pub struct RPM;

/// RPM 工具
impl RPM {
    pub fn is_src(file: &Path) -> bool {
        // use file
        let _type = Command::new("file")
                           .arg(file.to_str().unwrap())
                           .stdout(Stdio::piped())
                           .spawn()
                           .expect("Failed to execute file.");

        // grep key words
        let _grep = Command::new("grep")
                           .arg("RPM v.* src$")
                           .stdin(Stdio::from(_type.stdout.unwrap()))
                           .output()
                           .expect("failed to execute grep");


        _grep.status.success()
    }

    pub fn get_name(file: &Path) -> crate::Result<String> {
        let _output = Command::new("rpm")
                           .args(["-qp", "--qf", "%{name}", file.to_str().unwrap()])
                           .output()
                           .expect("Failed to excute rpm.");

        if _output.status.success() {
            String::from_utf8(_output.stdout).map_err(|e| e.into())
        } else {
            Err(String::from_utf8(_output.stderr).unwrap().into())
        }
    }

    pub fn get_version(file: &Path) -> crate::Result<String> {
        let _output = Command::new("rpm")
                           .args(["-qp", "--qf", "%{version}", file.to_str().unwrap()])
                           .output()
                           .expect("Failed to excute rpm.");

        if _output.status.success() {
            String::from_utf8(_output.stdout).map_err(|e| e.into())
        } else {
            Err(String::from_utf8(_output.stderr).unwrap().into())
        }
    }

    pub fn get_release(file: &Path) -> crate::Result<String> {
        let _output = Command::new("rpm")
                           .args(["-qp", "--qf", "%{release}", file.to_str().unwrap()])
                           .output()
                           .expect("Failed to excute rpm.");

        if _output.status.success() {
            String::from_utf8(_output.stdout).map_err(|e| e.into())
        } else {
            Err(String::from_utf8(_output.stderr).unwrap().into())
        }
    }

    pub fn install_src(file: &Path, workspace: &Workspace, package: &Package) -> crate::Result<()> {
        let _output = Command::new("rpm")
                           .arg("-i")
                           .arg("--define")
                           .arg(format!("_specdir {}", workspace.package_dir(package).to_str().unwrap()))
                           .arg("--define")
                           .arg(format!("_sourcedir {}", workspace.package_dir(package).to_str().unwrap()))
                           .arg(file.to_str().unwrap())
                           .output()
                           .expect("Failed to execute rpm.");

        if _output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8(_output.stderr).unwrap().into())
        }
    }
}
