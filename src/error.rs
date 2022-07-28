use std::fmt::{Formatter, Debug};

pub struct Error {
    pub message: String,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn new_error(message: String) -> Error {
    Error { message }
}