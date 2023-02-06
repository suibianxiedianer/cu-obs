use std::process::Command;

use crate::{
    IsOk,
    Package,
    workspace::Workspace,
};

/// OBS 系统，使用 osc 进行一些操作
pub struct OBS {
    workspace: Workspace,
}

impl OBS {
    /// 用于测试与 OBS 服务器的连接是否正常
    /// `osc api /about`
    pub fn alive() -> crate::Result<()> {
        let _output = Command::new("osc")
                             .args(["api", "/about"])
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 通过 `Package` 结构体，创建对应项目中的包
    pub fn init_pkg() -> crate::Result<()> {
        Ok(())
    }

    /// 是否存在此项目
    /// `osc api /source/[Project]/_meta`
    pub fn prj_exist(&self, prj: &str) -> crate::Result<()> {
        let _meta = format!("/source/{}/_meta", prj);
        let _output = Command::new("osc")
                             .args(["api", &_meta])
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 是否存在此软件
    /// `osc api /source/[Project]/[Package]/_meta`
    pub fn pkg_exist(&self, pkg: &Package) -> crate::Result<()> {
        let _meta = format!("/source/{}/{}/_meta", pkg.project, pkg.package);
        let _output = Command::new("osc")
                             .args(["api", &_meta])
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 同步 OBS 对应项目-包的代码
    /// `osc checkout [Project]/[Package]`
    pub fn checkout(&self, pkg: &Package) -> crate::Result<()> {
        let mut package_path = pkg.package.to_owned();
        package_path.push_str("/");
        package_path.push_str(pkg.package());

        let _output = Command::new("osc")
                             .args(["checkout", package_path.as_str()])
                             .current_dir(self.workspace.root())
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 同步 OBS 对应项目
    /// `osc checkout [Project]`
    // TODO or NOT: project 路径已存在会运行失败
    pub fn checkout_prj(&self, prj: &str) -> crate::Result<()> {
        let _output = Command::new("osc")
                             .args(["checkout", prj])
                             .current_dir(self.workspace.root())
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 切换至软件源码所在路径，执行 `add` 操作
    /// `osc add ./*`
    pub fn add_files(&self, pkg: &Package) -> crate::Result<()> {
        let _output = Command::new("osc")
                             .args(["add", "./*"])
                             .current_dir(self.workspace.package_dir(pkg))
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 在对应的包的家目录里执行 commit 命令
    /// `osc commit -m "MESSAGE"`
    pub fn commit(&self, pkg: &Package, message: String) -> crate::Result<()> {
        let _output = Command::new("osc")
                             .args(["commit", "-m", message.as_str()])
                             .current_dir(self.workspace.package_dir(pkg))
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 更新，拉取/推送 最新的源码，仅针对项目中的包
    /// `osc update`
    pub fn update(&self, pkg: &Package) -> crate::Result<()> {
        let _output = Command::new("osc")
                             .arg("update")
                             .current_dir(self.workspace.package_dir(pkg))
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }

    /// 更新，拉取/推送 最新的源码，操作对象为整体项目
    /// `osc update`
    pub fn update_prj(&self, prj: &str) -> crate::Result<()> {
        let mut root = self.workspace.root().to_owned();
        root.push(prj);

        let _output = Command::new("osc")
                             .arg("update")
                             .current_dir(root)
                             .output()
                             .expect("Failed to excute osc.");

        _output.is_ok()
    }
}
