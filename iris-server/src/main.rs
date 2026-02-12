mod config;
use anyhow::Result;
use quinn::{Endpoint, Connection};
use tracing::{info, error};

async fn handle(conn: Connection) {
    info!("Connected: {}", conn.remote_address());
    while let Ok((mut send, mut recv)) = conn.accept_bi().await {
        let _ = recv.read_to_end(1024*64).await.map(|d| info!("Received: {:?}", String::from_utf8_lossy(&d)));
        let _ = send.write_all(b"Ack").await;
        let _ = send.finish().await;
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = config::configure_server()?;
    let endpoint = Endpoint::server(config, "0.0.0.0:4433".parse()?)?;
    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            match conn.await {
                Ok(c) => handle(c).await,
                Err(e) => error!("Failed: {}", e),
            }
        });
    }
    Ok(())
}

// Tracing active
