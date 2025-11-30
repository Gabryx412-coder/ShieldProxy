use crate::config::LoggingConfig;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{ fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter };

pub mod metrics;

pub fn init(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup Tracing (Logging Strutturato)
    let format = fmt
        ::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact(); // O json() in base alla config, qui semplificato

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_|
        EnvFilter::new(&config.level)
    );

    if config.format == "json" {
        tracing_subscriber::registry().with(fmt::layer().json().with_filter(filter)).init();
    } else {
        tracing_subscriber
            ::registry()
            .with(fmt::layer().event_format(format).with_filter(filter))
            .init();
    }

    // 2. Setup Prometheus Exporter
    // Espone le metriche su una porta dedicata (es. 9090)
    // In produzione questo endpoint non dovrebbe essere pubblico
    let builder = PrometheusBuilder::new();
    let addr: SocketAddr = "0.0.0.0:9090".parse()?; // Porta fissa per le metrics o da config

    builder.with_http_listener(addr).install().expect("Failed to install Prometheus recorder");

    info!("Metrics exporter listening on {}", addr);

    Ok(())
}
