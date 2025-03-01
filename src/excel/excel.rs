use calamine::{DataType, Reader, Xlsx};
use std::{fs::File, io::Read};
use colored::Colorize;
use comfy_table::{Attribute, Cell, Color, Table};
use reqwest::blocking::get;
use std::io::Cursor;
use crate::processor::Processor;

#[derive(Debug)]
pub enum Operation {
    BasicInfo,
    Formatting,
    QuickView,
    BasicInfoWithHeader(usize),
}

pub struct ExcelFile {
    _content: Vec<u8>,
    workbook: Xlsx<Cursor<Vec<u8>>>,
    operation: Operation,
}

impl ExcelFile {
    // Create a new ExcelFile from raw bytes with specified operation
    pub fn new(content: Vec<u8>, operation: Operation) -> Result<Self, Box<dyn std::error::Error>> {
        let cursor = Cursor::new(content.clone());
        let workbook = Xlsx::new(cursor)?;
        
        Ok(ExcelFile {
            _content: content,
            workbook,
            operation,
        })
    }

    // Create from local file path
    pub fn from_path(path: &str, operation: Operation) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Self::new(content, operation)
    }

    // Create from URL
    pub fn from_url(url: &str, operation: Operation) -> Result<Self, Box<dyn std::error::Error>> {
        let response = get(url)?;
        let content = response.bytes()?.to_vec();
        Self::new(content, operation)
    }
    
    // Analyze formatting
    pub fn analyze_formatting(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Loop through sheets
        let sheet_names = self.workbook.sheet_names().to_vec();
        
        for (index, sheet_name) in sheet_names.iter().enumerate() {
            println!("{}", format!("Analysing sheet: {}", sheet_name).cyan());
            if let Some(Ok(range)) = self.workbook.worksheet_range_at(index) {
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
                    if !rows.is_empty() {
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
                }
                // Check for merged regions
                match self.workbook.load_merged_regions() {
                    Ok(()) => {
                        let merged_regions = self.workbook.merged_regions_by_sheet(sheet_name);
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

    // Quick view of data
    pub fn quick_view(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sheet_names = self.workbook.sheet_names().to_vec();

        for sheet_name in sheet_names {
            println!("Sheet: {}", sheet_name);
            println!("--------------------");

            match self.workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    let max_columns = 12; // Adjust this to show more or fewer columns
                    let max_rows = 10; // Maximum number of rows to display
                    let row_count = range.rows().count();
                    let mut table: Vec<Vec<String>> = Vec::new();

                    for (row_index, row) in range.rows().enumerate() {
                        if row_index >= max_rows {
                            break;
                        }
                        let row_data: Vec<String> = row
                            .iter()
                            .take(max_columns)
                            .map(|cell| cell.to_string().trim().to_owned())
                            .collect();
                        table.push(row_data);
                    }

                    // Calculate max width for each column
                    let max_widths: Vec<usize> = (0..max_columns)
                        .map(|col| {
                            table
                                .iter()
                                .map(|row| row.get(col).map_or(0, |cell| cell.len()))
                                .max()
                                .unwrap_or(0)
                                .min(10)
                        })
                        .collect();

                    // Print table
                    for row in table {
                        for (i, cell) in row.iter().enumerate() {
                            if i < max_widths.len() { // Ensure index is in bounds
                                if cell.len() > max_widths[i] {
                                    print!("{:.width$}..  ", cell, width = max_widths[i] - 2);
                                } else {
                                    print!("{:<width$}  ", cell, width = max_widths[i]);
                                }
                            }
                        }
                        println!();
                    }

                    // If there are more rows, indicate that
                    if row_count > max_rows {
                        println!(
                            "... only showing 10 rows out of approximately {}",
                            row_count
                        );
                    }
                }
                Err(e) => println!("Failed to read sheet: {}", e),
            }
            println!("\n");
        }
        Ok(())
    }

    // Display basic info about the Excel file
    pub fn display_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add sheet names to list
        let sheet_names = self.workbook.sheet_names().to_vec();

        // Loop through sheets
        for (index, sheet_name) in sheet_names.iter().enumerate() {
            if let Some(Ok(range)) = self.workbook.worksheet_range_at(index) {
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
                    let data_rows: Vec<_> = range.rows().skip(header_index + 1).take(10).collect();

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

    // Display basic info with specified header index
    pub fn display_info_with_header(&mut self, header_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        // Add sheet names to list
        let sheet_names = self.workbook.sheet_names().to_vec();

        // Loop through sheet names
        for (index, sheet_name) in sheet_names.iter().enumerate() {
            if let Some(Ok(range)) = self.workbook.worksheet_range_at(index) {
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

                // Use the provided header_index
                if let Some(header_row) = range.rows().nth(header_index) {
                    let column_count = header_row.len();
                    // Find the first non-empty row after the header then take 10 rows
                    let data_rows: Vec<_> = range.rows().skip(header_index + 1).take(10).collect();
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
                    println!(
                        "Could not find header row at index {} in the sheet",
                        header_index
                    );
                }
            } else {
                println!("Cannot read sheet: {}", sheet_name);
            }
        }
        Ok(())
    }
}

impl Processor for ExcelFile {
    fn process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.operation {
            Operation::BasicInfo => self.display_info(),
            Operation::Formatting => self.analyze_formatting(),
            Operation::QuickView => self.quick_view(),
            Operation::BasicInfoWithHeader(header_index) => self.display_info_with_header(*header_index),
        }
    }
}