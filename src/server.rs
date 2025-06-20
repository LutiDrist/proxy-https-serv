use hyper::service::service_fn;
use hyper::{Body, Client, Request, Response, StatusCode};
use hyper::client::HttpConnector;
use hyper_rustls::{ HttpsConnectorBuilder};
use std::sync::Arc;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use hyper_tls::HttpsConnector; // Removed to avoid type confusion
use log::info;
use prometheus::{default_registry, Encoder};
use crate::balancer::Balancer;
use crate::tls::load_tls_config;
use tokio_rustls::TlsAcceptor;
use crate::metrics::{record_request_duration, REQUEST_DURATION, REQUEST_COUNT};
pub async fn handle_request(
    req: Request<Body>,
    balancer: Arc<Balancer>,
    client: Client<hyper_rustls::HttpsConnector<HttpConnector>>,
) -> Result<Response<Body>, hyper::Error> {
    let backend = balancer.next_backend();
    let uri_str = format!("{}{}", backend, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("/"));

    let new_req = Request::builder()
        .method(req.method())
        .uri(uri_str)
        .body(req.into_body())
        .unwrap_or_else(|e| panic!("Failed to build request: {}", e)); // или return Err

    client.request(new_req).await
}

pub async fn metrics_handler() -> Result<Response<Body>, hyper::Error> {
    let encoder = prometheus::TextEncoder::new();
    let metric_families =  default_registry().gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain; version=0.0.4")
        .body(Body::from(buffer))
        .unwrap()
    )
}

pub async fn start_server(_config_path: &str, cert_path: &str, key_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting HTTPS server");

    let tls_config = load_tls_config(cert_path, key_path)?;
    let tls_acceptor = TlsAcceptor::from(tls_config);

    let addr: SocketAddr = "127.0.0.1:8443".parse()?;
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on https://{}", addr);

    let backends = vec![
        "http://127.0.0.1:8080".to_string(), // Временно HTTP для теста
        "http://127.0.0.1:8081".to_string(),
    ];
    let balancer = Arc::new(Balancer::new(backends));

    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build(); // Убрали https_or_http(), так как бэкенды HTTP
    let client = hyper::Client::builder().build::<_, Body>(https);
let service = {
    let balancer = Arc::clone(&balancer);
    let client = client.clone();
    service_fn(move |req: Request<Body>| {
        let balancer = Arc::clone(&balancer);
        let client = client.clone();
        async move {
            if req.uri().path() == "/metrics" {
                metrics_handler().await
            } else {
                handle_request(req, balancer, client).await
            }
        }
    })
};

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let service = service.clone();

        tokio::spawn(async move {
            let tls_stream = match acceptor.accept(stream).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("TLS handshake error: {:?}", e);
                    return;
                }
            };

            if let Err(err) = hyper::server::conn::Http::new()
                .serve_connection(tls_stream, service)
                .await
            {
                eprintln!("Connection error: {:?}", err);
            }
        });
    }
}