
use std::io;
use std::result;
use std::str;

pub type Result<T> = result::Result<T, FontError>;

#[derive(Debug)]
pub enum FontError {
    IOError(io::Error),
    Utf8Error(str::Utf8Error),
    DataUriDecodeError,
    FaceAlreadyAdded,
    FontInstanceAlreadyAdded,
    FaceNotFound,
    FaceNotLoaded,
    FaceFamilyNameMissing,
    FaceSizeMissing,
    FaceGlyphMissing
}

impl From<io::Error> for FontError {
    fn from(err: io::Error) -> Self {
        FontError::IOError(err)
    }
}

impl From<str::Utf8Error> for FontError {
    fn from(err: str::Utf8Error) -> Self {
        FontError::Utf8Error(err)
    }
}
