use nebby::json::analyze_json_nesting;

#[test]
fn test_simple_api_get_reqwest() {
    let url = "https://api.carbonintensity.org.uk/regional/regionid/1";

    // Fetch the file
    let result = analyze_json_nesting(url);

    // Assert that the fetch was successful
    assert!(result.is_ok(), "Failed to fetch the remote file");
}
