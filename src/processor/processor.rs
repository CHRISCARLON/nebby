pub trait Processor {
    fn process(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
