pub mod pg;
pub use pg::PgSource;

use tokio::sync::broadcast::Receiver;

use std::error::Error;

pub trait Source {
    fn new(shutdown_rx: Receiver<()>) -> Self;
    async fn read(&self) -> Result<Vec<String>, Box<dyn Error>>;
    async fn run(&self);
    async fn get_conn(&self) -> Result<deadpool_postgres::Object, Box<dyn std::error::Error>>;
}
