use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WafRulesDefinition {
    pub rules: Vec<WafRuleConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WafRuleConfig {
    pub id: String,
    pub name: String,
    pub category: String,
    pub severity: String,
    pub pattern: String,
    pub action: String, // "block", "allow"
}
