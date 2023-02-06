pub mod workspace;

pub(crate) mod rpm;
pub mod obs;
pub mod cmd;

pub const WORKSPACE: &str = "/tmp/cc";

#[derive(Debug)]
pub struct Package {
    project: String,
    package: String,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
