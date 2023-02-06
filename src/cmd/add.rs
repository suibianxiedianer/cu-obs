use std::process::{Command, Stdio};

use crate::Package;
use crate::workspace::Workspace;

#[derive(Debug)]
pub struct Add {
    file: String,
}

impl Add {
    pub(crate) fn new(file: impl ToString) -> Self {
        Add {
            file: file.to_string(),
        }
    }

    // TODO: current_dir not confirm
    pub(crate) fn apply(&self) -> crate::Result<()> {
        // release source code start
        // 1 : rpm2cpio
        let rpm2cpio = Command::new("rpm2cpio")
                              .arg(&self.file)
                              .stdout(Stdio::piped())
                              .spawn()
                              .expect("Failed to excute rpm2cpio.");

        // 2 : cpio
        let cpio = Command::new("cpio")
                              .arg("-div")
                              .stdin(Stdio::from(rpm2cpio.stdout.unwrap()))
                              .status()
                              .expect("Failed to execute cpio after rpm2cpio done.");
        // release source code done

        // add

        // commit
        Ok(())
    }
}
