use std::sync::atomic::{ AtomicUsize, Ordering };
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BackendSet {
    pub name: String,
    pub urls: Vec<String>,
    counter: Arc<AtomicUsize>,
}

impl BackendSet {
    pub fn new(name: String, urls: Vec<String>) -> Self {
        Self {
            name,
            urls,
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn next_url(&self) -> Option<String> {
        if self.urls.is_empty() {
            return None;
        }
        let idx = self.counter.fetch_add(1, Ordering::Relaxed);
        Some(self.urls[idx % self.urls.len()].clone())
    }
}
