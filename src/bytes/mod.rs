use reqwest::blocking::get;
use std::io::Read;

// Define filetypes
#[derive(Debug, PartialEq)]
pub enum FileType {
    PDF,
    PNG,
    JPEG,
    GIF,
    ZIP,
    XLSX,
    DOCX,
    XLS,
    PARQUET,
    CSV,
    Unknown,
}

// Get bytes
pub fn view_bytes(url: &str) -> Result<([u8; 100], FileType), Box<dyn std::error::Error>> {
    match get(url) {
        Ok(response) => {
            let mut buffer = [0u8; 100];
            match response.take(100).read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read < 100 {
                        buffer[bytes_read..].fill(0);
                    }
                    let file_type = identify_file_type(&buffer);
                    Ok((buffer, file_type))
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

// Filetype logic
fn identify_file_type(bytes: &[u8]) -> FileType {
    match bytes {
        // PDF magic number
        [0x25, 0x50, 0x44, 0x46, ..] => FileType::PDF,
        // PNG magic number
        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => FileType::PNG,
        // JPEG magic number
        [0xFF, 0xD8, 0xFF, ..] => FileType::JPEG,
        // GIF magic number
        [0x47, 0x49, 0x46, 0x38, ..] => FileType::GIF,
        // ZIP magic number (used for DOCX, XLSX, etc.)
        [0x50, 0x4B, 0x03, 0x04, rest @ ..] => match rest {
            [0x14, 0x00, 0x06, 0x00, ..] => FileType::XLSX,
            [0x14, 0x00, 0x08, 0x00, ..] => FileType::DOCX,
            _ => FileType::ZIP,
        },
        // Parquet magic number (first 4 bytes: PAR1)
        [0x50, 0x41, 0x52, 0x31, ..] => FileType::PARQUET,
        // Microsoft Compound File Binary Format (used for XLS, older DOC, etc.)
        [0xD0, 0xCF, 0x11, 0xE0, ..] => FileType::XLS,
        // Attempt to detect CSV by checking if the first 100 bytes seem to be comma-separated values
        _ if bytes
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b',' || b == b'\n' || b == b'\r') =>
        {
            FileType::CSV
        }
        // Default case for unknown file types
        _ => FileType::Unknown,
    }
}

// Fields to match on
pub fn get_file_type_string(file_type: &FileType) -> &'static str {
    match file_type {
        FileType::PDF => "PDF",
        FileType::PNG => "PNG",
        FileType::JPEG => "JPEG",
        FileType::GIF => "GIF",
        FileType::ZIP => "ZIP",
        FileType::XLSX => "Excel (XLSX)",
        FileType::DOCX => "Word (DOCX)",
        FileType::XLS => "Excel (XLS)",
        FileType::PARQUET => "Parquet",
        FileType::CSV => "CSV",
        FileType::Unknown => "Unknown",
    }
}
