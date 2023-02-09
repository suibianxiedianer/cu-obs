use crate::{workspace::Workspace, Package};

mod add;
pub use add::Add;

mod new;
pub use new::New;

mod status;
pub use status::Status;

mod binaries;
pub use binaries::Binaries;

mod unknown;
pub use unknown::Unknown;

#[derive(Debug)]
pub enum Command {
    Add(Add),
    New(New),
    Status(Status),
    Binaries(Binaries),
    Unknown(Unknown),
}

impl Command {
    pub fn from_args() -> crate::Result<Self> {
        Ok(Command::Unknown(Unknown::new("shiranai")))
    }

    /// 获取当前命令的字符串名
    pub fn get_name(&self) -> &str {
        use Command::*;

        match self {
            Add(_) => "add",
            New(_) => "new",
            Status(_) => "status",
            Binaries(_) => "Binaries",
            Unknown(cmd) => cmd.get_name(),
        }
    }

    // TODO: apply 需返回有用的信息，例字符串等
    pub fn apply(&self, pkg: &Package, ws: &Workspace) -> crate::Result<()> {
        use Command::*;

        match self {
            Add(cmd) => cmd.apply(pkg, ws),
            New(cmd) => cmd.apply(),
            Status(cmd) => cmd.apply(pkg),
            Binaries(cmd) => cmd.apply(pkg, ws),
            Unknown(cmd) => cmd.apply(),
        }
    }
}
