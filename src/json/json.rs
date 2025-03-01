use colored::Colorize;
use reqwest::blocking::get;
use serde_json::Value;
use std::error::Error;
use crate::processor::Processor;

pub struct JsonResponse {
    url: String,
    operation: Operation,
}

pub enum Operation {
    BasicJson,
}

impl JsonResponse {
    pub fn new(url: &str, operation: Operation) -> Self {
        Self {
            url: url.to_string(),
            operation,
        }
    }

    pub fn analyze(&self) -> Result<(), Box<dyn Error>> {
        // Call the api and fetch data
        let json = simple_api_get_reqwest(&self.url)?;

        // Print the color list
        print_color_list();

        // Calculate the nesting level
        let depth = calculate_nesting_level(&json, 0);
        let nesting_level = get_nesting_level(depth);

        // Print the nesting level and depth
        println!(
            "{} {}",
            "Nesting level:".blue().bold(),
            format!("{:?}", nesting_level).yellow()
        );
        println!(
            "{} {}",
            "Depth:".blue().bold(),
            format!("{}", depth).yellow()
        );

        // Print the structure of the JSON with colors
        println!("{}", "JSON Structure:".purple().bold());
        print_json_structure(&json, 0);

        Ok(())
    }
}

impl Processor for JsonResponse {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        match self.operation {
            Operation::BasicJson => self.analyze(),
        }
    }
}

// Define nesting levels
#[derive(Debug)]
enum NestingLevel {
    Flat,     // No nesting, or a very shallow structure
    Shallow,  // Slightly nested
    Moderate, // Moderately nested
    Deep,     // Quite nested
    VeryDeep, // Extremely nested
}

// Function to determine the depth of a JSON structure
fn calculate_nesting_level(json: &Value, current_depth: usize) -> usize {
    match json {
        Value::Object(map) => map
            .values()
            .map(|value| calculate_nesting_level(value, current_depth + 1))
            .max()
            .unwrap_or(current_depth),
        Value::Array(arr) => arr
            .iter()
            .map(|value| calculate_nesting_level(value, current_depth + 1))
            .max()
            .unwrap_or(current_depth),
        _ => current_depth,
    }
}

// Function to give a rating based on depth
fn get_nesting_level(depth: usize) -> NestingLevel {
    match depth {
        0 | 1 => NestingLevel::Flat,
        2 => NestingLevel::Shallow,
        3 => NestingLevel::Moderate,
        4 => NestingLevel::Deep,
        _ => NestingLevel::VeryDeep,
    }
}

// Function to print the list of colors for each level
fn print_color_list() {
    println!("{}", "Color List for Each Nesting Level:".purple().bold());
    for i in 0..7 {
        let colorized_text = get_color_for_level(i, &format!("Level {}: Color", i));
        println!("{}", colorized_text);
    }
}

// List of colors for different levels
fn get_color_for_level(level: usize, text: &str) -> colored::ColoredString {
    match level % 7 {
        0 => text.red(),
        1 => text.green(),
        2 => text.yellow(),
        3 => text.blue(),
        4 => text.magenta(),
        5 => text.cyan(),
        _ => text.white(),
    }
}

// Recursive function to print the structure of the JSON with colors
fn print_json_structure(json: &Value, indent: usize) {
    let colorized_brace = get_color_for_level(indent / 4, "{");
    match json {
        Value::Object(map) => {
            println!("{}{}", " ".repeat(indent), colorized_brace);
            for (key, value) in map {
                let colorized_key = get_color_for_level(indent / 4, key);
                print!("{}\"{}\": ", " ".repeat(indent + 2), colorized_key);
                print_json_structure(value, indent + 4);
            }
            let colorized_closing_brace = get_color_for_level(indent / 4, "}");
            println!("{}{}", " ".repeat(indent), colorized_closing_brace);
        }
        Value::Array(arr) => {
            let colorized_open_bracket = get_color_for_level(indent / 4, "[");
            println!("{}{}", " ".repeat(indent), colorized_open_bracket);
            for value in arr {
                print_json_structure(value, indent + 4);
            }
            let colorized_closing_bracket = get_color_for_level(indent / 4, "]");
            println!("{}{}", " ".repeat(indent), colorized_closing_bracket);
        }
        _ => {
            let colorized_value = get_color_for_level(indent / 4, &json.to_string());
            println!("{}{}", " ".repeat(indent), colorized_value);
        }
    }
}

// Function to call and API endpoint with GET and return JSON data
fn simple_api_get_reqwest(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let response = get(url)?;

    if response.status().is_success() {
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");

        let json: Value = if content_type.contains("application/json") {
            response.json()?
        } else {
            let bytes = response.bytes()?;
            serde_json::from_slice(&bytes).unwrap_or_else(|_| {
                serde_json::json!({
                    "raw_data": String::from_utf8_lossy(&bytes).into_owned()
                })
            })
        };

        println!("{}", "Received JSON:".green().bold());

        Ok(json)
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}
