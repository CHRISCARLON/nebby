use reqwest::blocking::get;
use std::fs::File;
use std::io::Read;
use crate::processor::Processor;

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

pub enum Operation {
    Nibble
}

pub struct FileBytes {
    bytes: [u8; 100],
    operation: Operation,
}

impl FileBytes {    
    pub fn from_url(url: &str, operation: Operation) -> Result<Self, Box<dyn std::error::Error>> {
        let response = get(url)?;
        let mut buffer = [0u8; 100];
        response.take(100).read(&mut buffer)?;
        Ok(FileBytes { bytes: buffer, operation })
    }

    pub fn from_path(path: &str, operation: Operation) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 100];
        file.read(&mut buffer)?;
        Ok(FileBytes { bytes: buffer, operation })
    }
    
    pub fn identify_type(&self) -> FileType {
        match &self.bytes {
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
            _ if self.bytes
                .iter()
                .all(|&b| b.is_ascii_alphanumeric() || b == b',' || b == b'\n' || b == b'\r') =>
            {
                FileType::CSV
            }
            // Default case for unknown file types
            _ => FileType::Unknown,
        }
    }

    pub fn get_bytes(&self) -> &[u8; 100] {
        &self.bytes
    }
}

impl Processor for FileBytes {
    fn process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.operation {
            Operation::Nibble => {
                let file_type = self.identify_type();
                let bytes = self.get_bytes();
                println!("File type: {:?}", file_type);
                println!("First 16 bytes (hex):");
                for (i, byte) in bytes.iter().take(16).enumerate() {
                    print!("{:02X} ", byte);
                    if (i + 1) % 8 == 0 {
                        println!();
                    }
                }
                Ok(())
            }
        }
    }
}
