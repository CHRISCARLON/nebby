use nebby::excel::ExcelFile;
use nebby::excel::Operation;

#[test]
fn test_excel_quick_view() {
    let url = "https://data.london.gov.uk/download/number-bicycle-hires/ac29363e-e0cb-47cc-a97a-e216d900a6b0/tfl-daily-cycle-hires.xlsx";

    // Fetch the file and handle the Result
    let mut excel_file = ExcelFile::from_url(url, Operation::QuickView).expect("Failed to create ExcelFile");

    // Now pass the content
    let result = excel_file.quick_view();

    assert!(result.is_ok(), "Quick view should execute without errors");
}
