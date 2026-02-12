mod config;
use anyhow::Result;
use quinn::Endpoint;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = config::configure_server()?;
    let endpoint = Endpoint::server(config, "127.0.0.1:4433".parse()?)?;
    info!("Listening on {}", endpoint.local_addr()?);
    Ok(())
}
