use crate::error::AppError;
// Importazione delle funzioni dal modulo metrics
use crate::observability::metrics::{track_ratelimit, track_request, track_waf_block};
use crate::proxy::load_balancer::BackendSet;
use crate::ratelimit::RateLimiter;
use crate::waf::engine::WafEngine;
use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, State},
    http::{Request, Response, StatusCode},
};
use http_body_util::BodyStream;
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info, instrument};

pub struct AppState {
    pub http_client: Client,
    pub waf: WafEngine,
    pub limiter: RateLimiter,
    pub backends: Vec<BackendSet>,
}

#[instrument(skip(state, req), fields(ip = %addr.ip(), uri = %req.uri()))]
pub async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut req: Request<Body>,
) -> Result<Response<Body>, AppError> {
    let start_time = Instant::now();
    let ip = addr.ip();
    let method = req.method().to_string();

    // 1. Rate Limiting Check
    if !state.limiter.check(ip) {
        info!("Rate Limit exceeded");
        track_ratelimit(&ip.to_string());
        track_request(&method, 429, start_time);
        
        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("Retry-After", "60")
            .body(Body::from("Rate limit exceeded"))
            .unwrap());
    }

    // 2. WAF Check
    let uri_path = req.uri().path();
    let query = req.uri().query().unwrap_or("");
    
    if let Some(rule_id) = state.waf.scan_request(uri_path, query) {
        info!(rule_id = %rule_id, "WAF Blocked Request");
        track_waf_block(&rule_id);
        track_request(&method, 403, start_time);

        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Request blocked by ShieldProxy WAF"))
            .unwrap());
    }

    // 3. Load Balancing
    let backend = state.backends.first().ok_or(AppError::BackendUnavailable)?;
    let target_base = backend.next_url().ok_or(AppError::BackendUnavailable)?;
    let target_url = format!("{}{}", target_base, req.uri());
    
    // Converte il Body di Axum in Bytes per reqwest
    let body_bytes = hyper::body::to_bytes(req.into_body()).await
        .map_err(|e| AppError::Internal(format!("Failed to read request body: {}", e)))?;

    let mut upstream_req = state.http_client
        .request(req.method().clone(), &target_url)
        .headers(req.headers().clone())
        .body(body_bytes) // Invia il body come Bytes
        .build()
        .map_err(AppError::Request)?;
        
    // Rimuovi header Host originale
    upstream_req.headers_mut().remove("host");

    // 4. Forwarding
    let upstream_res = state.http_client
        .execute(upstream_req)
        .await;

    match upstream_res {
        Ok(res) => {
            let status = res.status();
            track_request(&method, status.as_u16(), start_time);

            let mut response_builder = Response::builder().status(status);
            
            // Copia gli header dalla risposta del backend
            for (k, v) in res.headers() {
                response_builder = response_builder.header(k, v);
            }

            // Converte il Body di reqwest in un Body compatibile con Axum
            let body = Body::from_stream(BodyStream::new(res.bytes_stream()));

            Ok(response_builder.body(body).unwrap())
        }
        Err(e) => {
            error!(error = %e, "Upstream error");
            track_request(&method, 502, start_time);
            Err(AppError::Request(e))
        }
    }
}
