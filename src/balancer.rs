use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use log::info;

#[derive(Clone)]
pub struct Balancer {
    backends: Vec<String>,
    counter: Arc<AtomicUsize>,
}

impl Balancer {
    pub fn new(backends: Vec<String>) -> Self {
        info!("Creating balancer with {:?} backends", backends);
        Balancer{
            backends,
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn next_backend(&self) -> &str{
        let index = self.counter.fetch_add(1, Ordering::Relaxed) % self.backends.len();
        let backend = &self.backends[index];
        info!("Next backend: {}", backend);
        backend
    }
}


