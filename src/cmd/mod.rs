mod add;
pub use add::Add;

mod status;
pub use status::Status;

mod binaries;
pub use binaries::Binaries;

mod unknown;
pub use unknown::Unknown;

#[derive(Debug)]
pub enum Command {
    Add(Add),
    Status(Status),
    Binaries(Binaries),
    Unknown(Unknown),
}

impl Command {
    pub fn from_args() -> crate::Result<Self> {
        Ok(Command::Unknown(Unknown::new("shiranai")))
    }

    pub fn get_name(&self) -> &str {
        match self {
            Command::Add(_) => "add",
            Command::Status(_) => "status",
            Command::Binaries(_) => "Binaries",
            Command::Unknown(cmd) => cmd.get_name(),
        }
    }

    pub fn apply(&self) -> crate::Result<()> {
        use Command::*;

        match self {
            Add(cmd) => cmd.apply(),
            Status(cmd) => cmd.apply(),
            Binaries(cmd) => cmd.apply(),
            Unknown(cmd) => cmd.apply(),
        }
    }
}
