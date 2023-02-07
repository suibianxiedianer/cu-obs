pub mod cli;

pub(crate) mod workspace;

pub(crate) mod obs;
pub(crate) mod rpm;

pub mod cmd;

pub const WORKSPACE: &str = "/tmp/cc";

#[derive(Debug)]
pub struct Package {
    project: String,
    package: String,
}

impl Package {
    /// 从 prject 和 package 新建一个 Package
    pub fn new(project: impl ToString, package: impl ToString) -> Self {
        Package {
            project: project.to_string(),
            package: package.to_string(),
        }
    }

    /// 获取 project 字段
    pub fn project(&self) -> &str {
        &self.project
    }

    /// 获取 package 字段
    pub fn package(&self) -> &str {
        &self.package
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub trait IsOk {
    fn is_ok(self) -> crate::Result<()>;

    fn result(self) -> crate::Result<String>;
}

impl IsOk for std::process::Output {
    fn is_ok(self) -> crate::Result<()> {
        if self.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8(self.stderr).unwrap().into())
        }
    }

    fn result(self) -> crate::Result<String> {
        if self.status.success() {
            String::from_utf8(self.stdout).map_err(|e| e.into())
        } else {
            Err(String::from_utf8(self.stderr).unwrap().into())
        }
    }
}
