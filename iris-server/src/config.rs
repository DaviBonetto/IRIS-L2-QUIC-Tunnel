use iris_common::{make_server_cert, ALPN_QUIC_HTTP};
use quinn::{ServerConfig, TransportConfig};
use std::sync::Arc;
use anyhow::{Result, Context};

pub fn configure_server() -> Result<ServerConfig> {
    let (cert, key) = make_server_cert().map_err(|e| anyhow::anyhow!(e)).context("failed cert")?;
    let cert = rustls::Certificate(cert);
    let key = rustls::PrivateKey(key);
    let mut crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;
    crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
    let mut config = ServerConfig::with_crypto(Arc::new(crypto));
    config.transport_config(Arc::new(TransportConfig::default()));
    Ok(config)
}
