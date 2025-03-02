use nebby::bytes::FileBytes;
use nebby::bytes::Operation;
use nebby::bytes::FileType;

#[test]
fn test_pdf_file() {
    let url = "https://data.london.gov.uk/download/lfb-financial-and-performance-reporting-2024-25/091d06f8-cfad-4286-9263-45351822bb50/LFB%20KPI%20report%20-%20data%20up%20to%202024.06%20%28July%20report%29%20V3.2%20FB.pdf";

    let result = FileBytes::from_url(url, Operation::Nibble);

    assert!(result.is_ok(), "Failed to read in bytes");

    let file_bytes = result.unwrap();
    assert_eq!(file_bytes.identify_type(), FileType::PDF, "Expected PDF file type");
}
