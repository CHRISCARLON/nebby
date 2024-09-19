use nebby::csv::{fetch_remote_csv, process_basic_csv}; // Assuming `nebby::csv` is your module path

#[test]
fn test_print_csv_headers() {
    let url = "https://data.london.gov.uk/download/mpsantisocialbehaviour/5ee9e479-f719-4788-a233-ec26a295805f/MPS_Antisocial_Behaviour.csv"; // A valid CSV URL

    // Fetch the remote CSV content
    let bytes = fetch_remote_csv(url);

    // Ensure the CSV fetch succeeded
    assert!(bytes.is_ok(), "Failed to fetch CSV");

    // Unwrap the result to get the bytes
    let bytes = bytes.unwrap();

    // Attempt to print the headers from the CSV
    let result = process_basic_csv(bytes);

    // Check that reading and printing headers was successful
    assert!(result.is_ok(), "Failed to read and print CSV headers");
}
