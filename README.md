# Proxy HTTPS Server with Prometheus Metrics

Welcome to the **Proxy HTTPS Server** project! This is a Rust-based application that implements a reverse proxy server with load balancing and integrates Prometheus for performance monitoring. The server supports HTTPS with TLS, routes requests to multiple backends, and collects metrics such as request counts and processing times.

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Metrics](#metrics)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Overview
This project is a learning exercise and a practical implementation of a reverse proxy server written in Rust. It leverages the `hyper` framework for HTTP handling, `tokio` for asynchronous operations, and `prometheus` for monitoring. The server balances requests across multiple backends and exposes metrics via an `/metrics` endpoint, making it suitable for integration with tools like Grafana.

## Features
- **HTTPS Support:** Secure communication using TLS certificates.
- **Load Balancing:** Distributes incoming requests across multiple backend servers.
- **Prometheus Metrics:** Collects and exposes metrics such as total request count and request duration.
- **Configurable:** Supports custom configuration via a TOML file.
- **Logging:** Detailed logging for debugging and monitoring.

## Installation

### Prerequisites
- Rust (latest stable version, install via [rustup](https://rustup.rs/))
- Python 3.x (for running test backends)
- OpenSSL (for certificate generation and validation)

### Steps
1. **Clone the Repository:**
   
bash
   git clone https://github.com/your-username/proxy-https-serv.git
   cd proxy-https-serv
  

2. **Install Dependencies:**
   Ensure Rust and its tools are installed:
   
bash
   rustup update
  

3. **Generate TLS Certificates:**
   Create `cert.pem` and `key.pem` for HTTPS:
   
bash
   openssl req -x509 -newkey rsa:2048 -keyout key.pem -oBuild the Project:-nodes
  

4. **Build the Project:**
   
bash
   cargo build --release
  

## Usage

### Running the Server
1. **Configure the Server:**
   Edit `config.toml` to specify paths to your `cert.pem` and `key.pem`, and adjust the port if needed.

2. **Start the Server:**
   
bash
   cargo run --release
  
   Set the logging level for detailed output:
   
bash
   export RUST_LOG="info,proxy_httpStart Test Backends:un --release
  

3. **Start Test Backends:**
   Run simple HTTP servers on ports 8080 and 8081:
   
bash
   python -m http.serveAccess the Server:tp.server 8081
  

4. **Access the Server:**
   - Open `https://localhost:8443` in a browser (accept the self-signed certificate).
   - Check metrics at `https://localhost:8443/metrics`.

### Monitoring
- **Prometheus Integration:** Configure Prometheus to scrape metrics from `http://localhost:8443/metrics`.
- **Grafana Visualization:** Set up Grafana to visualize metrics (see [Development](#development) for setup instructions).

## Configuration
The server reads configuration from `config.toml`. Example:
toml
port = 8443
cert_path = "cert.pem"
key_path = "key.pem"
`

- `port`: The port to listen on (default: 8443).
- `cert_path`: Path to the TLS certificate file.
- `key_path`: Path to the TLS private key file.
## Metrics
The server exposes the following Prometheus metrics:
- proxy_requests_total: Total number of HTTP requests processed (counter).
- proxy_request_duration_seconds: Histogram of request processing times (in seconds).

To view metrics, send a GET request to /metrics endpoint over HTTPS.

## Development

### Running Locally
Follow the [Installation](#installation) steps and use cargo run for development.

### Adding Visualization with Grafana
1. Install Prometheus:
   Download from [Prometheus](https://prometheus.io/download/) and configure prometheus.yml:
  
YAML

   global:
     scrape_interval: 15s
   scrape_configs:
     - job_name: 'proxy-server'
       static_configs:
         - targets: ['localhost:8443']
       metrics_path: '/metrics'
       scheme: 'https'
   
   Run: .\prometheus.exe from the extracted folder.

2. Install Grafana:
   Download from [Grafana](https://grafana.com/get) and run: .\bin\grafana-server.exe.

3. Connect Grafana to Prometheus:
   - Open http://localhost:3000, log in (default: admin/admin).
   - Add a Prometheus data source with URL http://localhost:9090.

4. Create a Dashboard:
   - Add a panel with query rate(proxy_requests_total[5m]) for request rate.
   - Add a panel with query histogram_quantile(0.95, rate(proxy_request_duration_seconds_bucket[5m])) for 95th percentile latency.

### Testing
Use cargo test to run unit tests (add tests as needed).

## Contributing
Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create a feature branch (git checkout -b feature-name).
3. Commit your changes (git commit -m "Add feature").
4. Push to the branch (git push origin feature-name).
5. Open a Pull Request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements
- Thanks to the [Rust community](https://www.rust-lang.org/) for the amazing ecosystem.
- Inspired by tutorials and documentation from [Hyper](https://hyper.rs/) and [Prometheus](https://prometheus.io/).
- Special thanks to my mentor/brother for guiding me through this journey! 😄
