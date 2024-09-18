// src/main.rs
mod api;
mod bytes;
mod excel;
use api::simple_api_get_reqwest;
use bytes::view_bytes;
use clap::{Parser, Subcommand};
use colored::Colorize;
use excel::{
    analyze_excel_formatting, display_remote_basic_info,
    display_remote_basic_info_specify_header_idx, excel_quick_view, fetch_remote_file,
};

#[derive(Parser, Debug)]
#[command(author = "Christopher Carlon", version = "0.1.1", about = "Nebby - quickly review basic information about remote xlsx files and API GET requests", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display basic information about an Excel file
    Basic {
        /// URL of the Excel file
        url: String,
    },
    /// Check formatting of an Excel file
    Format {
        /// URL of the Excel file
        url: String,
    },
    /// Quick view of an Excel file
    QuickView {
        /// URL of the Excel file
        url: String,
    },
    /// Experimental basic information feature with specified header index
    BasicIdx {
        /// URL of the Excel file
        url: String,
        /// Index of the header row (0-based)
        #[arg(short, long, default_value = "0")]
        header_index: usize,
    },
    /// Experimental basic API request feature
    BasicJson {
        /// API Endpoint
        url: String,
    },
    /// Check bytes of any file
    Nibble {
        /// Url of the file
        url: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Basic { url } => process_excel(url, "basic info"),
        Commands::Format { url } => process_excel(url, "formatting"),
        Commands::QuickView { url } => process_excel(url, "quick view"),
        Commands::BasicIdx { url, header_index } => process_excel_with_header(url, *header_index),
        Commands::BasicJson { url } => process_json(url),
        Commands::Nibble { url } => process_view_bytes(url),
    }
}

fn process_json(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;
    println!("{}", "Processing JSON...".blue());
    simple_api_get_reqwest(url)
}

fn process_excel(url: &str, operation: &str) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;
    println!("{}", format!("Processing Excel {}...", operation).blue());
    let bytes = fetch_remote_file(url)?;
    match operation {
        "basic info" => display_remote_basic_info(bytes),
        "formatting" => analyze_excel_formatting(bytes),
        "quick view" => excel_quick_view(bytes),
        _ => Err("Unknown operation".into()),
    }
}

fn process_excel_with_header(
    url: &str,
    header_index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;
    println!(
        "{}",
        format!(
            "Processing Excel with header set at INDEX {}...",
            header_index
        )
        .blue()
    );
    let bytes = fetch_remote_file(url)?;
    display_remote_basic_info_specify_header_idx(bytes, header_index)
}

fn process_view_bytes(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;
    println!("{}", "Viewing first 100 bytes...".blue());
    let bytes = view_bytes(url)?;
    println!("First 100 bytes:");
    for (i, byte) in bytes.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
    Ok(())
}

fn validate_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if url.is_empty() {
        return Err("Error: URL cannot be empty".into());
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Error: URL must start with http:// or https://".into());
    }
    Ok(())
}
