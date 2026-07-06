use std::fs;
use std::io;

pub fn read(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
