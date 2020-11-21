
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

pub fn load_bytes<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>
{
    let mut file = File::open(path)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
