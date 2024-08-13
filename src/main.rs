// src/main.rs
mod excel_file_functions;
mod reqwest_functions;
use clap::{Parser, Subcommand};
use colored::Colorize;
use excel_file_functions::{
    analyze_excel_formatting, display_remote_basic_info,
    display_remote_basic_info_specify_header_idx, excel_quick_view,
};
use reqwest_functions::simple_api_get_reqwest;

#[derive(Parser, Debug)]
#[command(author = "Christopher Carlon", version = "0.1.0", about = "Excel Quick Scan - quickly review basic information about any xlsx file", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display basic information about the Excel file
    Basic {
        /// URL of the Excel file
        #[arg(short, long)]
        url: String,
    },
    /// Check formatting of the Excel file
    Format {
        /// URL of the Excel file
        #[arg(short, long)]
        url: String,
    },
    /// Quick view of the Excel file
    QuickView {
        /// URL of the Excel file
        #[arg(short, long)]
        url: String,
    },
    /// Experimental basic information feature - add --header-index int to the end
    BasicIdx {
        /// URL of the Excel file
        #[arg(short, long)]
        url: String,

        /// Index of the header row (0-based)
        #[arg(short, long, default_value = "0")]
        header_index: usize,
    },
    BasicJson {
        /// Api Endpoint
        #[arg(short, long)]
        url: String,
    },
}

// Define an enum to represent either Vec<u8> or String
enum ContentType {
    Bytes(Vec<u8>),
    String(String),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::BasicJson { url } => process_url(url, process_json, "json info"),
        Commands::Basic { url } => process_url(url, process_excel, "basic info"),
        Commands::Format { url } => process_url(url, process_excel_format, "formatting"),
        Commands::QuickView { url } => process_url(url, process_excel_quick_view, "quick view"),
        Commands::BasicIdx { url, header_index } => process_url(
            url,
            |content| process_excel_with_header(content, *header_index),
            "basic info experimental",
        ),
    }
}

fn process_json(content: ContentType) -> Result<(), Box<dyn std::error::Error>> {
    match content {
        ContentType::String(json_string) => simple_api_get_reqwest(&json_string),
        ContentType::Bytes(_) => Err("Expected JSON string, got bytes".into()),
    }
}

fn process_excel(content: ContentType) -> Result<(), Box<dyn std::error::Error>> {
    match content {
        ContentType::Bytes(bytes) => display_remote_basic_info(bytes),
        ContentType::String(_) => Err("Expected bytes, got JSON string".into()),
    }
}

fn process_excel_format(content: ContentType) -> Result<(), Box<dyn std::error::Error>> {
    match content {
        ContentType::Bytes(bytes) => analyze_excel_formatting(bytes),
        ContentType::String(_) => Err("Expected bytes, got JSON string".into()),
    }
}

fn process_excel_quick_view(content: ContentType) -> Result<(), Box<dyn std::error::Error>> {
    match content {
        ContentType::Bytes(bytes) => excel_quick_view(bytes),
        ContentType::String(_) => Err("Expected bytes, got JSON string".into()),
    }
}

fn process_excel_with_header(
    content: ContentType,
    header_index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    match content {
        ContentType::Bytes(bytes) => {
            display_remote_basic_info_specify_header_idx(bytes, header_index)
        }
        ContentType::String(_) => Err("Expected bytes, got JSON string".into()),
    }
}

fn process_url<F>(url: &str, process_fn: F, operation: &str)
where
    F: Fn(ContentType) -> Result<(), Box<dyn std::error::Error>>,
{
    if url.is_empty() {
        eprintln!("{}", "Error: URL cannot be empty".red());
        return;
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        eprintln!("{}", "Error: URL must start with http:// or https://".red());
        return;
    }

    let content = match reqwest::blocking::get(url) {
        Ok(response) => {
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("");

            if content_type.contains("application/json") {
                match response.text() {
                    Ok(text) => ContentType::String(text),
                    Err(e) => {
                        eprintln!("{}", format!("Error reading JSON response: {}", e).red());
                        return;
                    }
                }
            } else {
                match response.bytes() {
                    Ok(bytes) => ContentType::Bytes(bytes.to_vec()),
                    Err(e) => {
                        eprintln!(
                            "{}",
                            format!("Error reading response as bytes: {}", e).red()
                        );
                        return;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Error fetching from URL: {}", e).red());
            return;
        }
    };

    match process_fn(content) {
        Ok(_) => println!(
            "{}",
            format!("Successfully Processed {}!", operation).green()
        ),
        Err(e) => eprintln!("{}", format!("Error processing file content: {}", e).red()),
    }
}
