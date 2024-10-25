pub trait Sink {
    fn new() -> Self;
    async fn write(&self, records: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
}
