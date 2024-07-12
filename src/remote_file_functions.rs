// remote_file_functions.rs
use calamine::{DataType, Reader, Xlsx};
use colored::Colorize;
use comfy_table::{Attribute, Cell, Color, Table};
use reqwest::blocking::get;
use std::io::Cursor;

pub fn analyze_excel_formatting(content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Create workbook
    let mut workbook: Xlsx<_> = Xlsx::new(Cursor::new(content))?;

    for (index, sheet_name) in workbook.sheet_names().to_vec().iter().enumerate() {
        println!("{}", format!("Analysing sheet: {}", sheet_name).cyan());
        if let Some(Ok(range)) = workbook.worksheet_range_at(index) {
            let mut formatting_issues = Vec::new();
            // Check if sheet is empty
            if range.is_empty() {
                formatting_issues.push("There was an empty sheet".to_string());
            } else {
                // Check for a missing first row and / or no header
                if let Some((header_index, _)) = range
                    .rows()
                    .enumerate()
                    .find(|(_, row)| row.iter().filter(|cell| !cell.is_empty()).count() > 1)
                {
                    if header_index > 0 {
                        formatting_issues.push(format!(
                            "The header is not on the first row. Found on row {}",
                            header_index + 1
                        ));
                    }
                } else {
                    formatting_issues.push("No header row found at all ".to_string());
                }
                let rows: Vec<_> = range.rows().collect();
                // Check for inconsistent row lengths
                let first_row_len = rows[0].len();
                for (i, row) in rows.iter().enumerate() {
                    if row.len() != first_row_len {
                        formatting_issues.push(format!(
                            "Row {} has inconsistent length (expected {}, got {})",
                            i + 1,
                            first_row_len,
                            row.len()
                        ));
                    }
                }
            }
            // Check for merged regions
            match workbook.load_merged_regions() {
                Ok(()) => {
                    let merged_regions = workbook.merged_regions_by_sheet(sheet_name);
                    if !merged_regions.is_empty() {
                        formatting_issues.push(format!(
                            "Sheet contains {} merged region(s)",
                            merged_regions.len()
                        ));
                    }
                }
                Err(e) => {
                    formatting_issues.push(format!("Failed to load merged regions: {}", e));
                }
            }
            // Print formatting issues
            if formatting_issues.is_empty() {
                println!("{}", "No formatting issues found.".green());
            } else {
                println!("{}", "Formatting issues:".yellow());
                for issue in formatting_issues {
                    println!("- {}", issue);
                }
            }
        } else {
            println!("{}", "Failed to read sheet".red());
        }
    }
    Ok(())
}

pub fn display_remote_basic_info(content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Create workbook
    let mut workbook: Xlsx<_> = Xlsx::new(Cursor::new(content))?;

    // Add sheet names to list
    let sheet_names = workbook.sheet_names().to_vec();

    // Loop through sheet names
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
                    .fg(Color::DarkRed),
            ]);

            // Find the header row
            if let Some((header_index, header_row)) = range
                .rows()
                .enumerate()
                .find(|(_, row)| row.iter().filter(|cell| !cell.is_empty()).count() > 1)
            {
                let column_count = header_row.len();

                // Find the first non-empty row after the header then take 10 rows
                let data_rows: Vec<_> = range.rows().skip(header_index + 6).take(10).collect();

                for (index, header) in header_row.iter().enumerate() {
                    let data_types: Vec<_> = data_rows
                        .iter()
                        .filter_map(|row| row.get(index))
                        .map(|cell| {
                            if cell.is_empty() {
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
                            }
                        })
                        .collect();

                    let most_common_type = if data_types.is_empty() {
                        "No Data"
                    } else {
                        data_types
                            .iter()
                            .filter(|&t| *t != "Empty")
                            .max_by_key(|&t| data_types.iter().filter(|&r| r == t).count())
                            .unwrap_or(&"Empty")
                    };

                    table.add_row(vec![
                        Cell::new(format!("Column {}: {}", index + 1, header.to_string())),
                        Cell::new(most_common_type),
                    ]);
                }
                let row_count = range.rows().count() - (header_index + 1);
                println!("Sheet Name: {}", sheet_name);
                println!("Total number of columns: {}", column_count);
                println!("Total number of rows: {}", row_count);
                println!("{table}");
                // Print each row from data_rows
                for (row_index, row) in data_rows.iter().enumerate() {
                    print!("Data Row {}: ", row_index + 1);
                    for cell in row.iter() {
                        print!("{} ", cell);
                    }
                    println!();
                }
            } else {
                println!("Could not find header row in the sheet");
            }
        } else {
            println!("Cannot read sheet: {}", sheet_name);
        }
    }
    Ok(())
}

pub fn fetch_remote_file(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = get(url)?;
    let content = response.bytes()?.to_vec();
    Ok(content)
}
