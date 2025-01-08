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
    let (tx, _rx) = broadcast::channel(1);

    let server_rx = tx.subscribe();

    let signal_handle = tokio::spawn({
        async move {
            let sigint = signal::ctrl_c();
            let mut sigterm =
                unix::signal(SignalKind::terminate())
                .expect("failed to install signal handler");

            tokio::select! {
                _ = sigint => {},
                _ = sigterm.recv() => {},
            }
            info!("received termination signal");
            tx.send(()).unwrap();
        }
    });

    let pg_handle = tokio::spawn(async move {
        
    });

    let webserver_handle = tokio::spawn(async move {
        webserver::serve(server_rx).await;
    });

    tokio::join!(signal_handle, pg_handle, webserver_handle);
}
