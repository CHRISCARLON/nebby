use colored::Colorize;
use reqwest::blocking::get;
use serde_json::Value;

pub fn simple_api_get_reqwest(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Make a GET request using reqwest::blocking::get
    let response = get(url)?;

    // Check if the request was successful
    if response.status().is_success() {
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");

        let json: Value = if content_type.contains("application/json") {
            response.json()?
        } else {
            // If it's not explicitly JSON, try to parse the bytes as JSON
            let bytes = response.bytes()?;
            serde_json::from_slice(&bytes).unwrap_or_else(|_| {
                // If parsing as JSON fails, create a JSON object with the raw data
                serde_json::json!({
                    "raw_data": String::from_utf8_lossy(&bytes).into_owned()
                })
            })
        };

        // Print the entire JSON structure
        println!("{}", "Received JSON:".green().bold());
        println!("{}", serde_json::to_string_pretty(&json)?);

        Ok(())
    } else {
        // If the request was not successful, return an error
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}
