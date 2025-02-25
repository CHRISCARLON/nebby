use nebby::bytes::{view_bytes, FileType};

#[test]
fn test_view_bytes() {
    let url = "https://api.carbonintensity.org.uk/regional/regionid/1";

    let result = view_bytes(url);

    assert!(result.is_ok(), "Failed to read in bytes");
}

#[test]
fn test_pdf_file() {
    let url = "https://data.london.gov.uk/download/lfb-financial-and-performance-reporting-2024-25/091d06f8-cfad-4286-9263-45351822bb50/LFB%20KPI%20report%20-%20data%20up%20to%202024.06%20%28July%20report%29%20V3.2%20FB.pdf";

    let result = view_bytes(url);

    assert!(result.is_ok(), "Failed to read in bytes");

    let (_, file_type) = result.unwrap();
    assert_eq!(file_type, FileType::PDF, "Expected PDF file type");
}

// #[test]
// fn test_png_file() {
//     let url =
//         "https://www.freepik.com/free-photo/draught-beer-png-mug_13299896.htm#query=png&position=0&from_view=keyword&track=ais_hybrid&uuid=49669f7d-f2c1-42b1-b2db-f28289d33b25";

//     let result = view_bytes(url);

//     assert!(result.is_ok(), "Failed to read in bytes");

//     let (_, file_type) = result.unwrap();
//     assert_eq!(file_type, FileType::PNG, "Expected PNG file type");
// }

// #[test]
// fn test_jpeg_file() {
//     let url = "https://www.freepik.com/free-photo/transparent-colourful-autumn-leaves_5286206.htm#query=jpeg&position=20&from_view=keyword&track=ais_hybrid&uuid=169c2d48-7211-46d0-8097-8ee9cea5a8e4";

//     let result = view_bytes(url);

//     assert!(result.is_ok(), "Failed to read in bytes");

//     let (_, file_type) = result.unwrap();
//     assert_eq!(file_type, FileType::JPEG, "Expected JPEG file type");
// }

// #[test]
// fn test_gif_file() {
//     let url = "https://www.w3.org/People/mimasa/test/imgformat/img/w3c_home.gif";

//     let result = view_bytes(url);

//     assert!(result.is_ok(), "Failed to read in bytes");

//     let (_, file_type) = result.unwrap();
//     assert_eq!(file_type, FileType::GIF, "Expected GIF file type");
// }

#[test]
fn test_zip_file() {
    let url = "https://data.london.gov.uk/download/trend-based-population-projections/f042e4c5-9365-4d88-9a48-66a2158a2873/2021-based%20trend%20projections.zip";

    let result = view_bytes(url);

    assert!(result.is_ok(), "Failed to read in bytes");

    let (_, file_type) = result.unwrap();
    assert_eq!(file_type, FileType::ZIP, "Expected ZIP file type");
}

#[test]
fn test_xlsx_file() {
    let url = "https://datamillnorth.org/download/2o13g/8n0/February%202025%20HMO%20public%20register.xlsx";

    let result = view_bytes(url);

    assert!(result.is_ok(), "Failed to read in bytes");

    let (_, file_type) = result.unwrap();
    assert_eq!(file_type, FileType::XLSX, "Expected XLSX file type");
}

// #[test]
// fn test_docx_file() {
//     let url = "https://data.london.gov.uk/download/kingston-upon-thames-reduction-and-recycling-plan/91f911b4-c0eb-4725-8640-a37eaf72c9d8/RBK%20RRP%20-%20July%202024%20update.docx";

//     let result = view_bytes(url);

//     assert!(result.is_ok(), "Failed to read in bytes");

//     let (_, file_type) = result.unwrap();
//     assert_eq!(file_type, FileType::DOCX, "Expected DOCX file type");
// }

#[test]
fn test_xls_file() {
    let url = "https://data.london.gov.uk/download/diversity-london-report-data/4090b383-a418-4e2b-8592-75334244cbc7/diversity-in-london-data-2003.xls";

    let result = view_bytes(url);

    assert!(result.is_ok(), "Failed to read in bytes");

    let (_, file_type) = result.unwrap();
    assert_eq!(file_type, FileType::XLS, "Expected XLS file type");
}

// #[test]
// fn test_parquet_file() {
//     let url = "https://github.com/apache/parquet-testing/blob/master/data/alltypes_plain.parquet?raw=true";

//     let result = view_bytes(url);

//     assert!(result.is_ok(), "Failed to read in bytes");

//     let (_, file_type) = result.unwrap();
//     assert_eq!(file_type, FileType::PARQUET, "Expected Parquet file type");
// }

// #[test]
// fn test_csv_file() {
//     let url = "https://data.london.gov.uk/download/mpsantisocialbehaviour/5ee9e479-f719-4788-a233-ec26a295805f/MPS_Antisocial_Behaviour.csv";

//     let result = view_bytes(url);

//     assert!(result.is_ok(), "Failed to read in bytes");

//     let (_, file_type) = result.unwrap();
//     assert_eq!(file_type, FileType::CSV, "Expected CSV file type");
// }
