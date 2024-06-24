// src/local_file_functions.rs
use calamine::{open_workbook, Reader, Xlsx, DataType};
use comfy_table::{Table, Cell, Color, Attribute};
use std::path::Path;

pub fn display_basic_info<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let sheet_names = workbook.sheet_names().to_vec();

    for sheet_name in sheet_names {
        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
            let mut table = Table::new();
            table.load_preset(comfy_table::presets::UTF8_FULL);
            table.set_header(vec![
                Cell::new("Column Headers")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
                Cell::new("Data Type")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkRed)
            ]);

            let row_count = range.rows().count();
            let column_count = if let Some(header_row) = range.rows().next() {
                let count = header_row.len();

                for (index, cell) in header_row.iter().enumerate() {
                    let data_type = if cell.is_empty() {
                        "Empty"
                    } else if cell.is_int() {
                        "Integer"
                    } else if cell.is_float() {
                        "Float"
                    } else if cell.is_string() {
                        "String"
                    } else if cell.is_bool() {
                        "Boolean"
                    } else if cell.is_error() {
                        "Error"
                    } else {
                        "Unknown"
                    };

                    table.add_row(vec![
                        Cell::new(format!("Column {}: {}", index + 1, cell.to_string())), 
                        Cell::new(data_type)
                    ]);
                }
                count
            } else {
                0
            };

            println!("Sheet Name: {}", sheet_name);
            println!("Total number of columns: {}", column_count);
            println!("Total number of rows: {}", row_count);
            println!("{table}");
        } else {
            println!("Cannot read sheet: {}", sheet_name);
        }
    }

    Ok(())
}