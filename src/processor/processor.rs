
pub trait ExcelProcessor {
    fn process(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}