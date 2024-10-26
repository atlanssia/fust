pub mod pg;
pub use pg::PgSource;

use std::error::Error;

pub trait Source {
    fn new() -> Self;
    async fn read(&self) -> Result<Vec<String>, Box<dyn Error>>;
}
