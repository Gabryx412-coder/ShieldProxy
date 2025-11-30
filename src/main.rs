mod config;
mod error;
mod proxy;
mod waf;
mod ratelimit;
mod observability;
mod dashboard;
// mod dashboard; // Step successivo

use crate::config::AppConfig;
use crate::proxy::handler::{ proxy_handler, AppState };
use crate::proxy::load_balancer::BackendSet;
use crate::ratelimit::RateLimiter;
use crate::waf::engine::WafEngine;
use axum::{ routing::any, Router };
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{ info, warn };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load Config
    let config = AppConfig::load().expect("Failed to load configuration");

    // 2. Init Observability (Logs & Metrics)
    observability::init(&config.logging)?;
    info!("Starting ShieldProxy...");

    // 3. Initialize Modules
    let waf = WafEngine::new(config.waf.enabled, &config.waf.mode, &config.waf.rules_file)?;
    let limiter = RateLimiter::new(
        config.ratelimit.enabled,
        config.ratelimit.requests_per_window,
        config.ratelimit.window_seconds,
        config.ratelimit.whitelist
    );

    let backends: Vec<BackendSet> = config.backends
        .iter()
        .map(|b| BackendSet::new(b.name.clone(), b.urls.clone()))
        .collect();

    // 4. HTTP Client
    let http_client = reqwest::Client
        ::builder()
        .timeout(Duration::from_millis(config.backends.first().map_or(5000, |b| b.timeout_ms)))
        .danger_accept_invalid_certs(true) // Utile per backend self-signed interni
        .build()?;

    let app_state = Arc::new(AppState { http_client, waf, limiter, backends });

    // 5. Router
    let app = Router::new()
        // Rotte Dashboard
        .route("/dashboard", axum::routing::get(dashboard_page_handler))
        .route("/api/stats", axum::routing::get(api_stats_handler))
        // Rotta Proxy (Catch-all)
        .route("/*path", any(proxy_handler))
        .with_state(app_state);

    // 6. Server Start (TLS vs HTTP)
    if config.server.tls.enabled {
        info!(
            "TLS Enabled. Loading certs from: {} and {}",
            config.server.tls.cert_path,
            config.server.tls.key_path
        );

        // Caricamento certificati
        let tls_config = RustlsConfig::from_pem_file(
            &config.server.tls.cert_path,
            &config.server.tls.key_path
        ).await.expect("Failed to load TLS certificates. Run 'make certs' to generate them.");

        info!("ShieldProxy listening on HTTPS {}", addr);
        axum_server
            ::bind_rustls(addr, tls_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>()).await?;
    } else {
        warn!("TLS Disabled. Running in plain HTTP mode (NOT RECOMMENDED for production).");
        info!("ShieldProxy listening on HTTP {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
    }

    Ok(())
}
