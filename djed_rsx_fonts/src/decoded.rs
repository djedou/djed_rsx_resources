
use std::rc::Rc;

use djed_rsx_shared::traits::font_traits::TEncodedFont;

use types::FontResourceData;

#[derive(Debug, PartialEq)]
pub struct DecodedFont {
    pub bytes: Rc<Vec<u8>>,
    pub face_index: usize
}

impl DecodedFont {
    pub fn from_raw_parts(bytes: Rc<Vec<u8>>, face_index: usize) -> DecodedFont {
        DecodedFont { bytes, face_index }
    }

    pub fn from_encoded_font<E>(encoded: &E, face_index: usize) -> DecodedFont
    where
        E: TEncodedFont
    {
        Self::from_raw_parts(Rc::clone(encoded.bytes().unwrap()), face_index)
    }

    pub fn info(&self) -> FontResourceData {
        FontResourceData {
            bytes: &self.bytes,
            face_index: self.face_index
        }
    }
}
