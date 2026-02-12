mod config;
use anyhow::Result;
use quinn::Endpoint;
use tracing::{info, error};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = config::configure_server()?;
    let endpoint = Endpoint::server(config, "0.0.0.0:4433".parse()?)?;
    info!("Listening...");
    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            match conn.await {
                Ok(c) => info!("Connected: {}", c.remote_address()),
                Err(e) => error!("Failed: {}", e),
            }
        });
    }
    Ok(())
}
