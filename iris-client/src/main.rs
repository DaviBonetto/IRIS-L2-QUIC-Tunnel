mod config;
use anyhow::Result;
use quinn::Endpoint;
use tracing::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(config::configure_client());
    let conn = endpoint.connect("127.0.0.1:4433".parse()?, "localhost")?.await?;
    let (mut send, mut recv) = conn.open_bi().await?;
    send.write_all(b"Hello from Titan").await?;
    send.finish().await?;
    let mut buf = Vec::new();
    recv.read_to_end(&mut buf).await?;
    info!("Received: {}", String::from_utf8_lossy(&buf));
    conn.close(0u32.into(), b"done");
    endpoint.wait_idle().await;
    Ok(())
}
