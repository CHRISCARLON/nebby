use nebby::json::JsonResponse;
use nebby::processor::Processor;
use nebby::json::Operation;

#[test]
fn test_simple_api_get_reqwest() {
    let url = "https://api.carbonintensity.org.uk/regional/regionid/1";

    // Fetch the file
    let result = JsonResponse::new(url, Operation::BasicJson).process();

    // Assert that the fetch was successful
    assert!(result.is_ok(), "Failed to fetch the remote file");
}
