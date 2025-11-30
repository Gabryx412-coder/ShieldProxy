use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub backends: Vec<BackendConfig>,
    pub waf: WafConfig,
    pub ratelimit: RateLimitConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls: TlsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BackendConfig {
    pub name: String,
    pub urls: Vec<String>,
    pub timeout_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WafConfig {
    pub enabled: bool,
    pub mode: String, // "block" or "monitor"
    pub rules_file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_window: u32,
    pub window_seconds: u64,
    pub whitelist: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config
            ::builder()
            // Default settings
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8443)?
            // Merge file principale
            .add_source(config::File::with_name("config/config.example.yaml"))
            // Override locale se esiste
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            // Override da variabili d'ambiente (es. APP_SERVER__PORT=9000)
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
