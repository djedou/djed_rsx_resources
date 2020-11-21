
use std::io;
use std::result;

pub type Result<T> = result::Result<T, FileError>;

#[derive(Debug)]
pub enum FileError {
    IOError(io::Error),
    FileAlreadyAdded,
    FileNotFound
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::IOError(err)
    }
}
