// src/main.rs
mod remote_file_functions;
use clap::{Parser, Subcommand};
use colored::Colorize;
use remote_file_functions::{
    analyze_excel_formatting, display_remote_basic_info,
    display_remote_basic_info_specify_header_idx, excel_quick_view, fetch_remote_file,
};

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
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Basic { url } => process_url(url, display_remote_basic_info, "basic info"),
        Commands::Format { url } => process_url(url, analyze_excel_formatting, "formatting"),
        Commands::QuickView { url } => process_url(url, excel_quick_view, "quick view"),
        Commands::BasicIdx { url, header_index } => process_url(
            url,
            |content| display_remote_basic_info_specify_header_idx(content, *header_index),
            "basic info experimental",
        ),
    }
}

fn process_url<F>(url: &str, process_fn: F, operation: &str)
where
    F: Fn(Vec<u8>) -> Result<(), Box<dyn std::error::Error>>,
{
    if url.is_empty() {
        eprintln!("{}", "Error: URL cannot be empty".red());
        return;
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        eprintln!("{}", "Error: URL must start with http:// or https://".red());
        return;
    }
    match fetch_remote_file(url) {
        Ok(content) => match process_fn(content) {
            Ok(_) => println!(
                "{}",
                format!("Successfully Processed {}!", operation).green()
            ),
            Err(e) => eprintln!("{}", format!("Error processing file content: {}", e).red()),
        },
        Err(e) => eprintln!("{}", format!("Error fetching file from URL: {}", e).red()),
    }
}
