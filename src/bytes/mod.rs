use reqwest::blocking::get;
use std::io::Read;

#[derive(Debug)]
pub enum FileType {
    PDF,
    PNG,
    JPEG,
    GIF,
    ZIP,
    XLSX,
    DOCX,
    Unknown,
}

pub fn view_bytes(url: &str) -> Result<([u8; 100], FileType), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut buffer = [0u8; 100];
    let bytes_read = response.take(100).read(&mut buffer)?;
    if bytes_read < 100 {
        buffer[bytes_read..].fill(0);
    }
    let file_type = identify_file_type(&buffer);
    Ok((buffer, file_type))
}

fn identify_file_type(bytes: &[u8]) -> FileType {
    match bytes {
        [0x25, 0x50, 0x44, 0x46, ..] => FileType::PDF,
        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => FileType::PNG,
        [0xFF, 0xD8, 0xFF, ..] => FileType::JPEG,
        [0x47, 0x49, 0x46, 0x38, ..] => FileType::GIF,
        [0x50, 0x4B, 0x03, 0x04, rest @ ..] => match rest {
            [0x14, 0x00, 0x06, 0x00, ..] => FileType::XLSX,
            [0x14, 0x00, 0x08, 0x00, ..] => FileType::DOCX,
            _ => FileType::ZIP,
        },
        _ => FileType::Unknown,
    }
}

pub fn get_file_type_string(file_type: &FileType) -> &'static str {
    match file_type {
        FileType::PDF => "PDF",
        FileType::PNG => "PNG",
        FileType::JPEG => "JPEG",
        FileType::GIF => "GIF",
        FileType::ZIP => "ZIP",
        FileType::XLSX => "Excel (XLSX)",
        FileType::DOCX => "Word (DOCX)",
        FileType::Unknown => "Unknown",
    }
}
