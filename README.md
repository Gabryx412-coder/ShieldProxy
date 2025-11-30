# 🛡️ ShieldProxy: High-Performance Reverse Proxy & WAF in Rust

[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/Gabryx412-Coder/ShieldProxy?style=social)](https://github.com/Gabryx412-Coder/ShieldProxy)

ShieldProxy è un **Reverse Proxy asincrono ad alte prestazioni** con un **Web Application Firewall (WAF)** integrato, costruito interamente in **Rust** utilizzando l'ecosistema `tokio`/`axum`/`hyper`. Progettato per essere veloce, sicuro e pronto per l'ambiente di produzione Linux.

---

## ✨ Caratteristiche Principali

* **⚡ Performance & Concorrenza:** Basato su `tokio`, offre throughput elevato e bassa latenza.
* **🔒 TLS Termination:** Gestione nativa del traffico HTTPS/SSL con `rustls` per una sicurezza moderna e veloce.
* **🔥 Web Application Firewall (WAF):** Motore basato su Regex configurabile per mitigare attacchi comuni come **SQLi, XSS e Path Traversal**. Le regole sono ricaricabili a caldo.
* **✋ Rate Limiting:** Protezione DDoS e contro gli attacchi brute-force con limitazione delle richieste per IP.
* **📊 Osservabilità:** Esportazione di metriche in formato **Prometheus** (`/metrics`) e logging strutturato (JSON) con `tracing`.
* **🔄 Load Balancing:** Inoltro delle richieste verso i backend configurati con strategia Round-Robin.

---

## 🛠️ Architettura e Flusso di Lavoro

ShieldProxy intercetta il traffico in ingresso, esegue controlli di sicurezza e inoltra la richiesta al backend corretto. 

1.  **Ingresso HTTPS:** Il proxy termina la connessione TLS.
2.  **Rate Limiting:** Verifica che l'IP sorgente non abbia superato i limiti configurati.
3.  **WAF Inspection:** La richiesta (URI, Query, Headers) viene scansionata rispetto alle regole di sicurezza.
4.  **Load Balancing:** Se la richiesta è pulita, viene inoltrata a un backend disponibile (Round-Robin).
5.  **Logging:** Ogni evento (richiesta, blocco WAF, hit Rate Limit) è registrato in formato JSON.

---

## 🚀 Guida Rapida

### Prerequisiti

* Rust stable (edizione 2021)
* `openssl` (necessario per generare i certificati di test)

### 1. Clonazione e Setup

```bash
git clone [https://github.com/Gabryx412-Coder/ShieldProxy.git](https://github.com/Gabryx412-Coder/ShieldProxy.git)

cd ShieldProxy
