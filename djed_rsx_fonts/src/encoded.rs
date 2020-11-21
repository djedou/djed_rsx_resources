
use std::rc::Rc;

use djed_base64_util;
use djed_rsx_shared::traits::font_traits::TEncodedFont;

use error::{FontError, Result};
use types::FontEncodedData;

#[derive(Debug, PartialEq)]
pub enum EncodedFont {
    Bytes {
        bytes: Rc<Vec<u8>>
    },
    BytesAndDataUri {
        bytes: Rc<Vec<u8>>,
        data_uri: Rc<String>
    }
}

impl TEncodedFont for EncodedFont {
    type Error = FontError;

    fn from_bytes<T>(bytes: T) -> Result<Self>
    where
        T: Into<Rc<Vec<u8>>>
    {
        let bytes = bytes.into();
        Ok(EncodedFont::Bytes { bytes })
    }

    fn from_data_uri<T>(data_uri: T) -> Result<Self>
    where
        T: Into<Rc<String>>
    {
        let data_uri = data_uri.into();
        let bytes = Rc::new(djed_base64_util::from_data_uri(&data_uri).map_err(|_| FontError::DataUriDecodeError)?);
        Ok(EncodedFont::BytesAndDataUri { bytes, data_uri })
    }

    fn bytes(&self) -> Option<&Rc<Vec<u8>>> {
        match self {
            &EncodedFont::Bytes { ref bytes } | &EncodedFont::BytesAndDataUri { ref bytes, .. } => Some(bytes)
        }
    }

    fn data_uri(&self) -> Option<&Rc<String>> {
        match self {
            &EncodedFont::Bytes { .. } => None,
            &EncodedFont::BytesAndDataUri { ref data_uri, .. } => Some(data_uri)
        }
    }

    fn info(&self) -> FontEncodedData {
        match self {
            &EncodedFont::Bytes { ref bytes } => FontEncodedData::Bytes { bytes },
            &EncodedFont::BytesAndDataUri { ref data_uri, .. } => FontEncodedData::DataUri { data_uri }
        }
    }
}
