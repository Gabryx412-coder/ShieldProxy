use shield_proxy::waf::engine::WafEngine;
use std::path::PathBuf;

#[test]
fn test_waf_engine_blocking() {
    // Assicurati che rules.example.yaml sia accessibile (percorso relativo alla root del progetto)
    let mut rules_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rules_path.push("config/rules.example.yaml");

    let waf = WafEngine::new(true, "block", rules_path.to_str().unwrap()).unwrap();

    // Test 1: SQL Injection (SQLi)
    let sqli_uri = "/login?user=admin%27+UNION+SELECT+1,2,3";
    let sqli_query = "user=admin%27+UNION+SELECT+1,2,3";
    assert!(waf.scan_request(sqli_uri, sqli_query).is_some(), "SQLi should be blocked");

    // Test 2: Cross-Site Scripting (XSS)
    let xss_uri = "/search?q=<script>alert(1)</script>";
    let xss_query = "q=<script>alert(1)</script>";
    assert!(waf.scan_request(xss_uri, xss_query).is_some(), "XSS should be blocked");

    // Test 3: Path Traversal
    let path_traversal_uri = "/image?file=../../../../etc/passwd";
    let path_traversal_query = "file=../../../../etc/passwd";
    assert!(
        waf.scan_request(path_traversal_uri, path_traversal_query).is_some(),
        "Path Traversal should be blocked"
    );

    // Test 4: Richiesta Legittima
    let clean_uri = "/products/view?id=123";
    let clean_query = "id=123";
    assert!(waf.scan_request(clean_uri, clean_query).is_none(), "Clean request should be allowed");
}
