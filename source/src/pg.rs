use std::collections::HashMap;

use crate::Source;

pub struct PgSource {
    // client: Client,
    table: String,
    model: HashMap<String, String>,
}

impl Source for PgSource {
    fn new() -> Self {
        // let (client, connection) = tokio_postgres::connect(config.database.source_connection, tokio::runtime::Handle::current()).await.unwrap();
        //         tokio::spawn(connection);
        //         PgSource {
        //             client,
        //             table_name: config.source.table.to_string(),
        //             model: model.build_model(),
        //         }
        todo!()
    }

    async fn read(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }
}
