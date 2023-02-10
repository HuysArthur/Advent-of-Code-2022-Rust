use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn read_file(path: &str) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    buffer = buffer.trim().to_owned();
    Ok(buffer)
}