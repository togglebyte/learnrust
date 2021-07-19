// Creating a custom module

use std::io;

// Type alias
pub type Result<T> = std::result::Result<T, Error>;

// Enums
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

// Trait implementation
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
