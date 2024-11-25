use std::sync::{Arc, Mutex};

use tokio::{
    signal::{
        self,
        unix::{self, SignalKind},
    },
    sync::broadcast,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("starting");

    // register signal handler for termination signals
    let (tx, mut rx) = broadcast::channel(1);
    let rx = Arc::new(Mutex::new(rx));

    let signal_handle = tokio::spawn({
        let rx = Arc::clone(&rx);
        async move {
            let sigint = signal::ctrl_c();
            let mut sigterm =
                unix::signal(SignalKind::terminate()).expect("failed to install signal handler");
            tokio::select! {
                _ = sigint => {},
                _ = sigterm.recv() => {},
            }
            tx.send(()).unwrap();
        }
    });

    let server_rx = Arc::clone(&rx);
    let sever_handle = tokio::spawn(webserver::serve(server_rx));
}
