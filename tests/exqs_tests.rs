use exqs::remote_file_functions::{
    analyze_excel_formatting, display_remote_basic_info, excel_quick_view, fetch_remote_file,
};

#[test]
fn test_excel_quick_view() {
    // Test URL for a valid remote Excel file
    let url = "https://datamillnorth.org/download/2o13g/c476394e-8294-4c15-b1ff-44d32e6809c2/06.2024.%20.xlsx";

    // First, fetch the file
    let fetch_result = fetch_remote_file(url);
    assert!(fetch_result.is_ok(), "Failed to fetch the remote file");

    // If fetch was successful, proceed to quick view
    if let Ok(content) = fetch_result {
        // Call excel_quick_view and check if it's Ok - check no errors are returned.
        // This is only a basic test to begin with - needs to be made better in the future.
        let quick_view_result = excel_quick_view(content);
        assert!(
            quick_view_result.is_ok(),
            "Failed to perform quick view of Excel file"
        );
    }
}

#[test]
pub fn test_analyze_excel_formatting() {
    // Test URL for a valid remote Excel file
    let url = "https://data.london.gov.uk/download/mopac-grants-awarded/5ce18260-f2bb-4c80-8669-df06ab041445/MOPAC%20Q4%20Upload.xlsx";

    // First, fetch the file
    let fetch_result = fetch_remote_file(url);
    assert!(fetch_result.is_ok(), "Failed to fetch the remote file");

    // If fetch was successful, proceed to analyze formatting
    if let Ok(content) = fetch_result {
        let analysis_result = analyze_excel_formatting(content);
        // Assert that the analysis was successful
        assert!(
            analysis_result.is_ok(),
            "Failed to analyze Excel formatting"
        );
    }
}

#[test]
fn test_display_remote_basic_info() {
    // Test URL for a valid remote Excel file
    let url = "https://datamillnorth.org/download/2o13g/c476394e-8294-4c15-b1ff-44d32e6809c2/06.2024.%20.xlsx";

    // First, fetch the file
    let fetch_result = fetch_remote_file(url);
    assert!(fetch_result.is_ok(), "Failed to fetch the remote file");

    // If fetch was successful, proceed to display basic info
    if let Ok(content) = fetch_result {
        let display_result = display_remote_basic_info(content);
        assert!(
            display_result.is_ok(),
            "Failed to display basic info for the fetched file"
        );
    }
}

#[test]
fn test_file_fetch() {
    let url = "https://datamillnorth.org/download/2o13g/c476394e-8294-4c15-b1ff-44d32e6809c2/06.2024.%20.xlsx";

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
