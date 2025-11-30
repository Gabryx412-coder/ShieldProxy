use shield_proxy::ratelimit::RateLimiter;
use std::net::Ipv4Addr;
use std::time::Duration;

#[tokio::test]
async fn test_rate_limiter_fixed_window() {
    let max_req = 3;
    let window_sec = 2; // Finestra di 2 secondi
    let limiter = RateLimiter::new(true, max_req, window_sec, vec![]);
    let ip: std::net::IpAddr = Ipv4Addr::new(192, 168, 1, 1).into();

    // 1. Prime 3 richieste OK
    assert!(limiter.check(ip), "Request 1 should pass");
    assert!(limiter.check(ip), "Request 2 should pass");
    assert!(limiter.check(ip), "Request 3 should pass");

    // 2. Quarta richiesta bloccata
    assert!(!limiter.check(ip), "Request 4 should be blocked");

    // 3. Aspetta il reset della finestra (2 secondi)
    tokio::time::sleep(Duration::from_secs(window_sec + 1)).await;

    // 4. Nuova richiesta passata
    assert!(limiter.check(ip), "Request 5 should pass after reset");
}

#[test]
fn test_rate_limiter_whitelist() {
    let ip_wl: std::net::IpAddr = Ipv4Addr::new(10, 0, 0, 5).into();
    let limiter = RateLimiter::new(true, 1, 60, vec![ip_wl.to_string()]);

    // Whitelisted IP dovrebbe passare sempre, anche se il limite è 1
    assert!(limiter.check(ip_wl), "Whitelisted IP should pass (1)");
    assert!(limiter.check(ip_wl), "Whitelisted IP should pass (2)");
}
