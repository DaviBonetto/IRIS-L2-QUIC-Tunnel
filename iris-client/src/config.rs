use quinn::{ClientConfig, TransportConfig};
use std::sync::Arc;
struct SkipVerify;
impl rustls::client::ServerCertVerifier for SkipVerify {
    fn verify_server_cert(
        &self, _e: &rustls::Certificate, _i: &[rustls::Certificate],
        _s: &rustls::ServerName, _sc: &mut dyn Iterator<Item = &[u8]>,
        _o: &[u8], _n: std::time::SystemTime
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
pub fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipVerify))
        .with_no_client_auth();
    let mut cfg = ClientConfig::new(Arc::new(crypto));
    cfg.transport_config(Arc::new(TransportConfig::default()));
    cfg
}
