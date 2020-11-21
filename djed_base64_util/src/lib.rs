
extern crate base64;

pub fn to_image_data_uri(format: &str, bytes: &[u8]) -> String {
    let encoded = base64::encode(bytes);
    format!("data:image/{};base64,{}", format, encoded)
}

pub fn to_font_data_uri(bytes: &[u8]) -> String {
    let encoded = base64::encode(bytes);
    format!("data:application/x-font-woff;base64,{}", encoded)
}

pub fn from_data_uri(data_uri: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let start = data_uri.find("base64,").unwrap_or(0) + 7;
    base64::decode(&data_uri.as_bytes()[start..])
}
