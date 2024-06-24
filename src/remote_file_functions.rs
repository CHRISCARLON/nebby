use calamine::{Xlsx, Reader};
use comfy_table::{Table, Cell, Color, Attribute};
use reqwest::blocking::get;
use std::io::Cursor;

pub fn display_remote_basic_info(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Download the file into memory
    let response = get(url)?;
    let content = response.bytes()?;
    
    // Create Xlsx object from memory
    let mut workbook: Xlsx<_> = Xlsx::new(Cursor::new(content))?;

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
