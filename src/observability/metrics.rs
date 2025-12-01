use metrics::{histogram, increment_counter};
use std::time::Instant;

pub const HTTP_REQUESTS_TOTAL: &str = "shield_requests_total";
pub const HTTP_REQUEST_DURATION: &str = "shield_request_duration_seconds";
pub const WAF_BLOCKS: &str = "shield_waf_blocks_total";
pub const RATE_LIMIT_HITS: &str = "shield_ratelimit_hits_total";

/// Traccia la durata della richiesta HTTP e incrementa il contatore totale.
pub fn track_request(method: &str, status: u16, start: Instant) {
    let duration = start.elapsed().as_secs_f64();

    // Istogramma latenza (Sintassi corretta: nessun separatore tra value e la prima label key/value)
    histogram!(
        HTTP_REQUEST_DURATION,
        duration // <-- NESSUNA VIRGOLA QUI!
        "method" => method.to_string(),
        "status" => status.to_string() 
    );

    // Contatore richieste totali
    increment_counter!(
        HTTP_REQUESTS_TOTAL,
        "method" => method.to_string(),
        "status" => status.to_string()
    );
}

/// Traccia un blocco causato dal WAF.
pub fn track_waf_block(rule_id: &str) {
    increment_counter!(
        WAF_BLOCKS,
        "rule_id" => rule_id.to_string()
    );
}

/// Traccia un blocco causato dal Rate Limiter.
pub fn track_ratelimit(ip: &str) {
    increment_counter!(
        RATE_LIMIT_HITS,
        "ip" => ip.to_string()
    );
}
