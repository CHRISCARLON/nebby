use bytes::Bytes;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use csv::ReaderBuilder;
use reqwest::blocking::get;
use std::error::Error;
use std::io::Cursor;

// Fetch the remote CSV as bytes
pub fn fetch_remote_csv(url: &str) -> Result<Bytes, Box<dyn Error>> {
    let response = get(url)?;
    let content = response.bytes()?;
    Ok(content)
}

// Infer the type of a CSV value based on a sample value
fn infer_type(value: &str) -> &str {
    if value.parse::<i64>().is_ok() {
        "Integer"
    } else if value.parse::<f64>().is_ok() {
        "Float"
    } else if value.parse::<bool>().is_ok() {
        "Boolean"
    } else {
        "String"
    }
}

// Read CSV headers and infer data types, using ComfyTable to display results
pub fn process_basic_csv(csv_content: Bytes) -> Result<(), Box<dyn Error>> {
    // Create a cursor to read the bytes (since csv::Reader expects something that implements Read)
    let mut rdr = ReaderBuilder::new().from_reader(Cursor::new(csv_content));

    // Read the headers
    let headers = rdr.headers()?.clone();

    // Get the first row to infer data types
    let mut records = rdr.records();
    if let Some(result) = records.next() {
        let first_row = result?;

        // Create a comfy table for headers and their corresponding types
        let mut type_table = Table::new();
        type_table.load_preset(UTF8_FULL);
        type_table.set_header(vec!["Header", "Data Type"]);

        for (header, value) in headers.iter().zip(first_row.iter()) {
            let data_type = infer_type(value);
            type_table.add_row(vec![Cell::new(header), Cell::new(data_type)]);
        }

        // Print the headers and data type table
        println!("{}", type_table);

        // Create another table for the first 10 rows of data
        let mut data_table = Table::new();
        data_table.load_preset(UTF8_FULL);

        // Set headers for the data table
        data_table.set_header(headers.iter().map(|h| Cell::new(h)));

        // Add the first row
        data_table.add_row(first_row.iter().map(|value| Cell::new(value)));

        // Add up to 9 more rows (total 10)
        for (i, record) in records.enumerate() {
            if i >= 9 {
                break; // Stop after 10 rows (including the first row)
            }
            let row = record?;
            data_table.add_row(row.iter().map(|value| Cell::new(value)));
        }

        // Print the data table
        println!("\nFirst 10 rows of the CSV:");
        println!("{}", data_table);
    } else {
        println!("No data rows available to infer types.");
    }

    Ok(())
}
