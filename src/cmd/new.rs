#[derive(Debug)]
pub struct New {
    uri: String,
}

impl New {
    pub fn new(uri: impl ToString) -> Self {
        New {
            uri: uri.to_string(),
        }
    }

    pub fn apply(&self) -> crate::Result<()> {
        Ok(())
    }
}
