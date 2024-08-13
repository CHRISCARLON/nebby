use exqs::excel_file_functions::{
    analyze_excel_formatting, display_remote_basic_info,
    display_remote_basic_info_specify_header_idx, excel_quick_view, fetch_remote_file,
};

#[test]
fn test_file_fetch() {
    let url = "https://data.london.gov.uk/download/licensed-vehicles-numbers-borough/45c47aba-682d-4be4-b62a-42215203c2ad/Copy%20of%20vehicles-licensed-borough%20%281%29.xls";

    // Fetch the file
    let fetch_result = fetch_remote_file(url);

    // Assert that the fetch was successful
    assert!(fetch_result.is_ok(), "Failed to fetch the remote file");

    // If the fetch was successful, we can add more specific checks
    if let Ok(content) = fetch_result {
        // Check that we actually got some data
        assert!(!content.is_empty(), "Fetched file content is empty");
    }
}

#[test]
fn test_display_remote_basic_info() {
    let url = "https://data.london.gov.uk/download/ratio-house-prices-earnings-borough/122ea18a-cb44-466e-a314-e0c62a32529e/ratio-house-price-earnings-residence-based.xlsx";

    // Fetch the file and handle the Result
    let content = fetch_remote_file(url).expect("Failed to fetch remote file");

    // Now pass the content
    let result = display_remote_basic_info(content);

    assert!(result.is_ok());
    // Add more specific assertions based on the expected output
}

#[test]
fn test_analyze_excel_formatting() {
    let url = "https://data.london.gov.uk/download/unemployment-rate-region/8a29ec0c-9de3-4777-832f-49ef8c2b4d14/unemployment-region.xlsx";

    // Fetch the file and handle the Result
    let content = fetch_remote_file(url).expect("Failed to fetch remote file");

    // Now pass the content
    let result = analyze_excel_formatting(content);

    assert!(result.is_ok());
    // Add more specific assertions based on the expected output
}

#[test]
fn test_excel_quick_view() {
    let url = "https://data.london.gov.uk/download/number-bicycle-hires/ac29363e-e0cb-47cc-a97a-e216d900a6b0/tfl-daily-cycle-hires.xlsx";

    // Fetch the file and handle the Result
    let content = fetch_remote_file(url).expect("Failed to fetch remote file");

    // Now pass the content
    let result = excel_quick_view(content);

    assert!(result.is_ok());
    // Add more specific assertions based on the expected output
}

#[test]
fn test_display_remote_basic_info_specify_header_idx() {
    let url = "https://data.london.gov.uk/download/number-bicycle-hires/ac29363e-e0cb-47cc-a97a-e216d900a6b0/tfl-daily-cycle-hires.xlsx";

    // Fetch the file and handle the Result
    let content = fetch_remote_file(url).expect("Failed to fetch remote file");

    // Now pass the content
    let result = display_remote_basic_info_specify_header_idx(content, 3);

    assert!(result.is_ok());
    // Add more specific assertions based on the expected output
}
