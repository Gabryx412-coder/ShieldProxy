use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{ Duration, Instant };

#[derive(Debug, Clone)]
pub struct RateLimiter {
    // Map: IP -> (Count, WindowStart)
    store: Arc<DashMap<IpAddr, (u32, Instant)>>,
    max_requests: u32,
    window_duration: Duration,
    whitelist: Vec<String>, // Semplificato per l'esempio
    enabled: bool,
}

impl RateLimiter {
    pub fn new(enabled: bool, max_req: u32, window_sec: u64, whitelist: Vec<String>) -> Self {
        Self {
            store: Arc::new(DashMap::new()),
            max_requests: max_req,
            window_duration: Duration::from_secs(window_sec),
            whitelist,
            enabled,
        }
    }

    pub fn check(&self, ip: IpAddr) -> bool {
        if !self.enabled {
            return true;
        }

        // Check Whitelist (banale controllo stringhe per ora)
        if self.whitelist.iter().any(|w| w == &ip.to_string()) {
            return true;
        }

        let mut entry = self.store.entry(ip).or_insert((0, Instant::now()));
        let (count, window_start) = entry.value_mut();

        if window_start.elapsed() > self.window_duration {
            // Reset finestra
            *count = 1;
            *window_start = Instant::now();
            return true;
        }

        if *count >= self.max_requests {
            return false; // Limit exceeded
        }

        *count += 1;
        true
    }
}
