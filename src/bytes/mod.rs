use reqwest::blocking::get;
use std::io::Read;

pub fn view_bytes(url: &str) -> Result<[u8; 100], Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut buffer = [0u8; 100];
    let bytes_read = response.take(100).read(&mut buffer)?;

    if bytes_read < 100 {
        // If less than 100 bytes were read, fill the rest with zeros
        buffer[bytes_read..].fill(0);
    }

    Ok(buffer)
}
