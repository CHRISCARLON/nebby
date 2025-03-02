mod json;
mod bytes;
mod csv;
mod delta_lake;
mod excel;
mod utils;
mod parquet;
mod processor;
use json::JsonResponse;
use bytes::FileBytes;
use clap::{Parser, Subcommand};
use csv::{fetch_remote_csv, process_basic_csv};
use delta_lake::{get_aws_config, load_remote_delta_lake_table_info};
use excel::ExcelFile;
use parquet::ParquetFile;
use processor::Processor;
use tokio;
use utils::{create_progress_bar, validate_url};

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
        path: String,
        /// Path to the local file
        #[arg(short, long, help = "If true, the file will be processed locally")]
        local: bool,
    },
    /// Basic CSV feature
    BasicCsv { url: String },
    /// Process Delta Lake table from AWS S3
    DeltaLake {
        /// S3 URI of the Delta Lake table
        #[arg(short, long)]
        s3_uri: String,
    },
    /// Basic Parquet feature
    BasicParquet { path: String },
}

// Call commands and File Logic
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::BasicXl { path, local } => {
            let pb = create_progress_bar("Processing Excel basic info...");
            let result = match local {
                true => ExcelFile::from_path(path, excel::Operation::BasicInfo)?.process(),
                false => {
                    validate_url(path)?;
                    ExcelFile::from_url(path, excel::Operation::BasicInfo)?.process()
                }
            };
            pb.finish_with_message("Excel Processed");
            result
        },
        Commands::FormatXl { path, local } => {
            let pb = create_progress_bar("Processing Excel formatting...");
            let result = match local {
                true => ExcelFile::from_path(path, excel::Operation::Formatting)?.process(),
                false => {
                    validate_url(path)?;
                    ExcelFile::from_url(path, excel::Operation::Formatting)?.process()
                }
            };
            pb.finish_with_message("Excel Processed");
            result
        },
        Commands::QuickViewXl { path, local } => {
            let pb = create_progress_bar("Processing Excel quick view...");
            let result = match local {
                true => ExcelFile::from_path(path, excel::Operation::QuickView)?.process(),
                false => {
                    validate_url(path)?;
                    ExcelFile::from_url(path, excel::Operation::QuickView)?.process()
                }
            };
            pb.finish_with_message("Excel Processed");
            result
        },
        Commands::BasicIdxXl { path, header_index, local } => {
            let pb = create_progress_bar(&format!("Processing Excel with header at INDEX {}...", header_index));
            let result = match local {
                true => ExcelFile::from_path(path, excel::Operation::BasicInfoWithHeader(*header_index))?.process(),
                false => {
                    validate_url(path)?;
                    ExcelFile::from_url(path, excel::Operation::BasicInfoWithHeader(*header_index))?.process()
                }
            };
            pb.finish_with_message("Excel processing with header complete");
            result
        },
        Commands::BasicJson { url } => {
            let pb = create_progress_bar("Processing JSON...");
            let result = JsonResponse::new(url, json::Operation::BasicJson).process();
            pb.finish_with_message("JSON Processed");
            result
        },
        Commands::Nibble { path, local  } => {
            let pb = create_progress_bar("Processing Nibble...");
            let result = match local {
                true => FileBytes::from_path(path, bytes::Operation::Nibble)?.process(),
                false => {
                    validate_url(path)?;
                    FileBytes::from_url(path, bytes::Operation::Nibble)?.process()
                }
            };
            pb.finish_with_message("Nibble Processed");
            result
        },
        Commands::BasicCsv { url } => process_csv(url),
        Commands::DeltaLake { s3_uri } => process_delta_lake(s3_uri).await,
        Commands::BasicParquet { path } => {
            let pb = create_progress_bar("Processing Parquet...");
            let result = ParquetFile::new(path).process();
            pb.finish_with_message("Parquet Processed");
            result
        },
    }
}

// File Logic
// TODO: all this logic needs to be moved into its own crate 
// TODO: remove validate url doesn't need to be called in every remote file function - not needed
// TODO: add local flag to all commands that could support it
// TODO: combine all the process_ functions into one

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

