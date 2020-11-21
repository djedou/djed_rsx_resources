
use std::io;
use std::result;

use image;

pub type Result<T> = result::Result<T, ImageError>;

#[derive(Debug)]
pub enum ImageError {
    IOError(io::Error),
    LibError(image::ImageError),
    DataUriDecodeError,
    ImageAlreadyAdded
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> Self {
        ImageError::IOError(err)
    }
}

impl From<image::ImageError> for ImageError {
    fn from(err: image::ImageError) -> Self {
        ImageError::LibError(err)
    }
}
