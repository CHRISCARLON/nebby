// src/main.rs

mod local_file_functions;
mod remote_file_functions;
use local_file_functions::display_basic_info;
use remote_file_functions::display_remote_basic_info;
use clap::Parser;
use std::path::PathBuf;

/// A program to display basic Excel file information
#[derive(Parser, Debug)]
#[command(author = "Chris Carlon", version = "0.1", about = "Displays Basic Info About Excel Files", long_about = None)]
struct Args {
    /// Path to the Excel file
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// URL of the Excel file
    #[arg(short, long)]
    url: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(file) = args.file {
        if let Err(e) = display_basic_info(&file) {
            eprintln!("Error processing file: {}", e);
        }
    } else if let Some(url) = args.url {
        if let Err(e) = display_remote_basic_info(&url) {
            eprintln!("Error processing URL: {}", e);
        }
    } else {
        eprintln!("Please provide either a file path or a URL.");
    }
}
