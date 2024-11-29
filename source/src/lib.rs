pub mod pg;
use tokio::sync::broadcast::Receiver;

use std::error::Error;

pub trait Source {
    fn new(shutdown_rx: Receiver<()>) -> Self;
    async fn read(&self) -> Result<Vec<String>, Box<dyn Error>>;
    async fn run(&self);
    async fn get_conn(&self);
}
