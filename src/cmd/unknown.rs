#[derive(Debug)]
pub struct Unknown {
    command: String
}

impl Unknown {
    pub(crate) fn new(command: impl ToString) -> Self {
        Unknown {
            command: command.to_string(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.command
    }

    pub(crate) fn apply(&self) -> crate::Result<()> {
        Ok(())
    }
}
