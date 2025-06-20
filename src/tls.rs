use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use log::info;
use rustls_pemfile::{read_all, Item};

pub fn load_tls_config(cert_path: &str, key_path: &str) -> Result<Arc<ServerConfig>, Box<dyn std::error::Error>> {
    info!("Loading TLS configuration from cert_path={} and key_path={}", cert_path, key_path);

    let cert_file = File::open(cert_path)?;
    let mut cert_reader = BufReader::new(cert_file);
    let cert_chain: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_reader)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    if cert_chain.is_empty() {
        return Err("No valid certificate chain".into());
    }
    info!("Loaded {} certificates", cert_chain.len());

    let key_file = File::open(key_path)?;
    let mut key_reader = BufReader::new(key_file);
    let items: Vec<Result<Item, std::io::Error>> = read_all(&mut key_reader).into_iter().collect();
    let mut keys: Vec<PrivateKeyDer> = Vec::new();
    for item in items {
        match item {
            Ok(Item::Pkcs8Key(key)) => keys.push(PrivateKeyDer::Pkcs8(key)),
            Ok(Item::Pkcs1Key(key)) => keys.push(PrivateKeyDer::Pkcs1(key)),
            _ => continue,
        }
    }
    if keys.is_empty() {
        return Err("No private key found in key.pem".into());
    }
    info!("Loaded private key");
    let key = keys.pop().ok_or("No private key found in key.pem")?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?;
    info!("TLS configuration created successfully");

    Ok(Arc::new(config))
}