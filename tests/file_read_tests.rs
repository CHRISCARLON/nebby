use exqs::local_file_functions::display_basic_info;
use exqs::remote_file_functions::display_remote_basic_info;
use std::path::PathBuf;

struct Args {
    file: PathBuf,
    url: String,
}

fn create_local_args(file_path: &str) -> Args {
    Args {
        file: PathBuf::from(file_path),
        url: String::new(),
    }
}

fn create_remote_args(url: &str) -> Args {
    Args {
        file: PathBuf::new(),
        url: String::from(url),
    }
}

#[test]
fn test_display_basic_info() {
    // Test with a valid local Excel file
    let result = display_basic_info(&create_local_args("tests/test_file.xlsx").file);
    assert!(result.is_ok());
}

#[test]
fn test_display_remote_column_headers() {
    // Test with a valid remote Excel file
    let result = display_remote_basic_info(&create_remote_args("https://datamillnorth.org/download/2o13g/c476394e-8294-4c15-b1ff-44d32e6809c2/06.2024.%20.xlsx").url);
    assert!(result.is_ok());
}
