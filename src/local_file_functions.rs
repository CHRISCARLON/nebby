// src/local_file_functions.rs

use calamine::{open_workbook, Reader, Xlsx};
use comfy_table::{Table, Cell, Color, Attribute};
use std::path::Path;

pub fn display_basic_info<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mut table = Table::new();
        table.load_preset(comfy_table::presets::UTF8_FULL);
        table.set_header(vec![
            Cell::new("Column Headers")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)
        ]);

        let row_count = range.rows().count();
        
        let column_count = if let Some(file) = range.rows().next() {
            let count = file.len();
            for (index, cell) in file.iter().enumerate() {
                table.add_row(vec![
                    format!("Column {}: {}", index + 1, cell.to_string())
                ]);
            }
            count
        } else {
            0
        };

        println!("\nTotal number of columns: {}", column_count);
        println!("\nTotal number of rows: {}", row_count);
        println!("{table}");
        Ok(())
    } else {
        Err("Cannot read sheet".into())
    }
}