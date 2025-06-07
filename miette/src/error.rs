use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct MietteError {
    details: String,
}

impl MietteError {
    pub fn new(msg: String) -> MietteError {
        MietteError { details: msg }
    }
}

impl fmt::Display for MietteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MietteError {
    fn description(&self) -> &str {
        &self.details
    }
}
