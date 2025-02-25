mod api;
mod bytes;
mod csv;
mod delta_lake;
mod excel;
mod utils;
use api::analyze_json_nesting;
use bytes::{get_file_type_string, view_bytes};
use clap::{Parser, Subcommand};
use csv::{fetch_remote_csv, process_basic_csv};
use delta_lake::{get_aws_config, load_remote_delta_lake_table_info};
use excel::{
    analyze_excel_formatting, display_basic_info,
    display_basic_info_specify_header_idx, excel_quick_view, fetch_remote_file, fetch_local_file
};
use tokio;
use utils::create_progress_bar;

#[derive(Parser, Debug)]
#[command(author = "Christopher Carlon", version = "0.1.5", about = "Nebby! Quickly review basic information about a range of different file formats", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Define Commands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Display basic information about an Excel file
    BasicXl {
        /// URL of the Excel file
        path: String,
        /// Path to the local Excel file
        #[arg(short, long, help = "If true, the file will be processed locally")]
        local: bool,
    },
    /// Check formatting of an Excel file
    FormatXl {
        /// URL of the Excel file
        path: String,
        /// Path to the local Excel file
        #[arg(short, long, help = "If true, the file will be processed locally")]
        local: bool,
    },
    /// Quick view of an Excel file
    QuickViewXl {
        /// URL of the Excel file
        path: String,
        /// Path to the local Excel file
        #[arg(short, long, help = "If true, the file will be processed locally")]
        local: bool,
    },
    /// Experimental basic information feature with specified header index
    BasicIdxXl {
        /// URL of the Excel file
        path: String,
        /// Index of the header row (0-based)
        #[arg(short, long, default_value = "0")]
        header_index: usize,
        /// Path to the local Excel file
        #[arg(short, long, help = "If true, the file will be processed locally")]
        local: bool,
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
    /// Basic CSV feature
    BasicCsv { url: String },
    /// Process Delta Lake table from AWS S3
    DeltaLake {
        /// S3 URI of the Delta Lake table
        #[arg(short, long)]
        s3_uri: String,
    },
}

// Call commands and File Logic
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::BasicXl { path, local } => process_excel(path, "basic info", *local),
        Commands::FormatXl { path, local } => process_excel(path, "formatting", *local),
        Commands::QuickViewXl { path, local} => process_excel(path, "quick view", *local),
        Commands::BasicIdxXl { path, header_index, local } => process_excel_with_header(path, *header_index, *local),
        Commands::BasicJson { url } => process_json(url),
        Commands::Nibble { url } => process_view_bytes(url),
        Commands::BasicCsv { url } => process_csv(url),
        Commands::DeltaLake { s3_uri } => process_delta_lake(s3_uri).await,
    }
}

// File Logic
// TODO: all this logic needs to be moved into its own crate 
// TODO: remove validate url doesn't need to be called in every remote file function - not needed
fn process_json(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;

    let pb = create_progress_bar("Processing JSON...");
    let result = analyze_json_nesting(url);

    pb.finish_with_message("JSON Processed");
    result
}

fn process_excel(url: &str, operation: &str, local: bool) -> Result<(), Box<dyn std::error::Error>> {
    let pb = create_progress_bar(&format!("Processing Excel {}...", operation));
    
    let result = match local {
        true => {
            let bytes = fetch_local_file(url)?;
            match operation {
                "basic info" => display_basic_info(bytes),
                "formatting" => analyze_excel_formatting(bytes),
                "quick view" => excel_quick_view(bytes),
                _ => Err("Unknown operation".into()),
            }
        },
        false => {
            validate_url(url)?;
            let bytes = fetch_remote_file(url)?;
            match operation {
                "basic info" => display_basic_info(bytes),
                "formatting" => analyze_excel_formatting(bytes),
                "quick view" => excel_quick_view(bytes),
                _ => Err("Unknown operation".into()),
            }
        }
    };

    pb.finish_with_message("Excel Processed");
    result
}

fn process_excel_with_header(
    url: &str,
    header_index: usize,
    local: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let pb = create_progress_bar(&format!(
        "Processing Excel with header set at INDEX {}...",
        header_index
    ));

    let result = match local {
        true => {
            let bytes = fetch_local_file(url)?;
            display_basic_info_specify_header_idx(bytes, header_index)
        },
        false => {
            validate_url(url)?;
            let bytes = fetch_remote_file(url)?;
            display_basic_info_specify_header_idx(bytes, header_index)
        }
    };      

    pb.finish_with_message("Excel processing with header complete");
    result
}

fn process_view_bytes(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;

    let pb = create_progress_bar("Viewing first 100 bytes...");
    let (bytes, file_type) = view_bytes(url)?;
    pb.finish_and_clear();

    println!("First 100 bytes:");
    for (i, byte) in bytes.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!("\nDetected file type: {}", get_file_type_string(&file_type));
    Ok(())
}

fn process_csv(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    validate_url(url)?;

    let pb = create_progress_bar("Processing CSV...");
    let bytes = fetch_remote_csv(url)?;
    let result = process_basic_csv(bytes);

    pb.finish_with_message("CSV Processed");
    result
}

async fn process_delta_lake(s3_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pb = create_progress_bar("Processing Delta Lake table...");

    match get_aws_config().await {
        Ok(config) => match load_remote_delta_lake_table_info(s3_uri, config).await {
            Ok(_table) => {
                pb.finish_with_message("Successfully loaded the Delta table");
                Ok(())
            }
            Err(e) => {
                pb.finish_with_message("Error loading the Delta table");
                eprintln!("Error loading the Delta table: {}", e);
                Err(e.into())
            }
        },
        Err(e) => {
            pb.finish_with_message("Error getting AWS configuration");
            eprintln!("Error getting AWS configuration: {}", e);
            Err(e.into())
        }
    }
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
