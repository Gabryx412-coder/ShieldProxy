use crate::error::AppError;
use crate::waf::rules::{ WafRuleConfig, WafRulesDefinition };
use regex::Regex;
use std::fs;
use tracing::{ info, warn };

#[derive(Debug, Clone)]
pub struct CompiledRule {
    pub id: String,
    pub name: String,
    pub pattern: Regex,
    pub action: String,
}

#[derive(Debug, Clone)]
pub struct WafEngine {
    rules: Vec<CompiledRule>,
    enabled: bool,
    block_mode: bool,
}

impl WafEngine {
    pub fn new(enabled: bool, mode: &str, rules_path: &str) -> Result<Self, AppError> {
        if !enabled {
            return Ok(Self {
                rules: vec![],
                enabled: false,
                block_mode: false,
            });
        }

        info!("Loading WAF rules from: {}", rules_path);
        let content = fs::read_to_string(rules_path).map_err(AppError::Io)?;
        let defs: WafRulesDefinition = serde_yaml
            ::from_str(&content)
            .map_err(|e| AppError::WafParse(e.to_string()))?;

        let mut compiled_rules = Vec::new();
        for rule in defs.rules {
            compiled_rules.push(CompiledRule {
                id: rule.id,
                name: rule.name,
                pattern: Regex::new(&rule.pattern)?,
                action: rule.action,
            });
        }

        info!("WAF loaded with {} rules", compiled_rules.len());

        Ok(Self {
            rules: compiled_rules,
            enabled: true,
            block_mode: mode == "block",
        })
    }

    /// Ispeziona una richiesta (URI e Query String per ora)
    /// Ritorna Some(rule_id) se bloccato, None se OK.
    pub fn scan_request(&self, uri: &str, query: &str) -> Option<String> {
        if !self.enabled {
            return None;
        }

        // Combina input per scan rapido
        let target = format!("{}?{}", uri, query);

        for rule in &self.rules {
            if rule.pattern.is_match(&target) {
                warn!("WAF Match: Rule {} ({}) triggered on {}", rule.id, rule.name, uri);

                if self.block_mode && rule.action == "block" {
                    return Some(rule.id.clone());
                }
            }
        }
        None
    }
}
