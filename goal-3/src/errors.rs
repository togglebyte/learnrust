use std::io::Error as IoError;
use std::str::Utf8Error;
use std::sync::mpsc::SendError;
use std::sync::PoisonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Utf8(Utf8Error),
    MutexErr,
    SendErr,
}

impl Error {
    pub fn print(&self) {
        use Error::*;
        match self {
            Io(e) => eprintln!("IO error: {:?}", e),
            Utf8(e) => eprintln!("Failed to parse utf8 : {:?}", e),
            MutexErr => eprintln!("Could not acquire lock. Unrecoverable"),
            SendErr => eprintln!("Sending or receiving half deallocated. Unrecoverable"),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::SendErr
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Self::MutexErr
    }
}
