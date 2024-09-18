use nebby::bytes::view_bytes;

#[test]
fn test_view_bytes() {
    let url = "https://api.carbonintensity.org.uk/regional/regionid/1";

    let result = view_bytes(url);

    assert!(result.is_ok(), "Failed to read in bytes")
}
