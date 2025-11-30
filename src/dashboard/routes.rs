use axum::{ extract::State, response::{ Html, IntoResponse, Json }, http::StatusCode };
use std::sync::Arc;
use crate::proxy::handler::AppState;
use serde::Serialize;

#[derive(Serialize)]
pub struct DashboardStats {
    total_requests: u64,
    waf_blocks: u64,
    ratelimit_hits: u64,
    backends_status: Vec<BackendStatus>,
}

#[derive(Serialize)]
pub struct BackendStatus {
    name: String,
    urls: Vec<String>,
}

// Handler per l'API JSON che estrae le statistiche
pub async fn api_stats_handler(State(state): State<Arc<AppState>>) -> Result<
    Json<DashboardStats>,
    StatusCode
> {
    // Nota: L'estrazione dei contatori da Prometheus è complessa (richiede l'endpoint HTTP /metrics).
    // Per un MVP, esponiamo solo i dati facili da raggiungere o mockup:

    let backends_status = state.backends
        .iter()
        .map(|b| BackendStatus {
            name: b.name.clone(),
            urls: b.urls.clone(),
        })
        .collect();

    // In un sistema reale, qui useremmo un'API per interrogare il Prometheus Exporter.
    // Simuliamo i valori di base per coerenza con il design.
    let stats = DashboardStats {
        total_requests: 0, // Dati in tempo reale andrebbero recuperati da un counter condiviso
        waf_blocks: 0,
        ratelimit_hits: 0,
        backends_status,
    };

    Ok(Json(stats))
}

// Handler per la pagina HTML della Dashboard (Minimalista, in-line)
pub async fn dashboard_page_handler() -> Html<String> {
    let html =
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>ShieldProxy Dashboard</title>
    <style>
        body { font-family: 'Inter', sans-serif; background-color: #f4f7f6; color: #333; margin: 0; padding: 20px; }
        .container { max-width: 900px; margin: auto; background: #fff; padding: 20px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
        h1 { color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px; }
        .stat-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-top: 20px; }
        .stat-card { background: #ecf0f1; padding: 15px; border-radius: 6px; text-align: center; }
        .stat-card h3 { margin: 0 0 5px 0; color: #3498db; font-size: 1.5em; }
        .stat-card p { margin: 0; font-size: 0.9em; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🛡️ ShieldProxy Status</h1>
        <div class="stat-grid">
            <div class="stat-card">
                <h3 id="total-requests">0</h3>
                <p>Total Requests</p>
            </div>
            <div class="stat-card">
                <h3 id="waf-blocks">0</h3>
                <p>WAF Blocks</p>
            </div>
            <div class="stat-card">
                <h3 id="ratelimit-hits">0</h3>
                <p>Rate Limit Hits</p>
            </div>
        </div>

        <h2>Backend Status</h2>
        <ul id="backend-list"></ul>
    </div>

    <script>
        document.addEventListener('DOMContentLoaded', () => {
            function updateStats() {
                fetch('/api/stats')
                    .then(response => response.json())
                    .then(data => {
                        document.getElementById('total-requests').textContent = data.total_requests;
                        document.getElementById('waf-blocks').textContent = data.waf_blocks;
                        document.getElementById('ratelimit-hits').textContent = data.ratelimit_hits;

                        const backendList = document.getElementById('backend-list');
                        backendList.innerHTML = '';
                        data.backends_status.forEach(backend => {
                            const li = document.createElement('li');
                            li.textContent = `${backend.name}: ${backend.urls.join(', ')}`;
                            backendList.appendChild(li);
                        });
                    })
                    .catch(error => console.error('Error fetching stats:', error));
            }
            updateStats();
            // Aggiorna ogni 5 secondi
            // setInterval(updateStats, 5000); 
        });
    </script>
</body>
</html>
    "#.to_string();

    Html(html)
}
