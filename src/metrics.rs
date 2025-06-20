use prometheus::{default_registry, register_histogram, register_int_counter, Histogram, IntCounter};
use std::time::Instant;

lazy_static::lazy_static! {
    pub static ref REQUEST_COUNT: IntCounter = register_int_counter!(
        "proxy_requests_total",
        "Total number of HTTP requests processed by the proxy"
    ).unwrap();

    pub static ref REQUEST_DURATION: Histogram = register_histogram!(
        "proxy_request_duration_seconds",
        "HTTP request durations"
    ).unwrap();
}

pub fn record_request_duration(start: Instant) {
    let duration = start.elapsed();
    REQUEST_DURATION.observe(duration.as_secs_f64());
}