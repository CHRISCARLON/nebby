// src/main.rs
mod remote_file_functions;

use clap::{Parser, Subcommand};
use colored::Colorize;
use remote_file_functions::{
    analyze_excel_formatting, display_remote_basic_info, fetch_remote_file,
};

#[derive(Parser, Debug)]
#[command(author = "Christopher Carlon", version = "0.1.0", about = "Excel Query and Statistics (EXQS) Tool", long_about = None)]
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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Basic { url } => process_url(url, display_remote_basic_info, "basic info"),
        Commands::Format { url } => process_url(url, analyze_excel_formatting, "formatting"),
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
