use rcgen::generate_simple_self_signed;
pub fn make_server_cert() -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names)?;
    Ok((cert.serialize_der()?, cert.serialize_private_key_der()))
}


pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
