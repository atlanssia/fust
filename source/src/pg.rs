use std::{collections::HashMap, time::Duration};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio::sync::broadcast::Receiver;
use tokio_postgres::{Client, NoTls};

use tracing::{error, info};

use crate::Source;

pub struct PgSource {
    pool: Pool,
    shutdown_rx: Receiver<()>,
}

/// Implement the Source trait for PgSource
impl Source for PgSource {
    fn new(shutdown_rx: Receiver<()>) -> Self {
        let mut cfg = Config::new();
        cfg.url = Some("".to_string());
        cfg.dbname = Some("".to_string());

        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        cfg.keepalives = Some(true);
        cfg.keepalives_idle = Some(Duration::from_secs(60));

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        
        PgSource { pool, shutdown_rx }
    }

    async fn read(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get_conn(&self) -> Result<deadpool_postgres::Object, Box<dyn std::error::Error>> {
        loop {
            match self.pool.get().await {
                Ok(conn) => return Ok(conn),
                Err(e) => {
                    eprintln!("Failed to get connection: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await; // 重试前等待5秒
                }
            }
        }
    }

    async fn run(&self) {
        // self.shutdown_rx.recv().await;
    }
}
