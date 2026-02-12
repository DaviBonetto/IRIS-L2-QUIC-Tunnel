mod config;
use anyhow::Result;
use quinn::Endpoint;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(config::configure_client());
    let conn = endpoint.connect("127.0.0.1:4433".parse()?, "localhost")?.await?;
    let (mut send, mut recv) = conn.open_bi().await?;
    info!("Stream opened");
    Ok(())
}
