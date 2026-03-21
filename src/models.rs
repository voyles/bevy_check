use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum TargetPlatform {
    Generic,
    Switch,
    Ps5,
    Xbox,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Critical, // Score 0, Block PR
    Warning,  // Score -10, Notify Dev
    Info,     // No score change
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleCondition {
    Always,
    FeatureEnabled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub crate_id: String,
    pub severity: Severity,
    pub condition: RuleCondition,
    pub feature: Option<String>,
    pub reason: String,
    pub remediation: String,
}

#[derive(Debug, Default)]
pub struct AuditReport {
    pub portability_score: u8,
    pub violations: Vec<Violation>,
}

#[derive(Debug)]
pub struct Violation {
    pub crate_name: String,
    pub severity: Severity,
    pub message: String,
    pub help: String,
    pub dependency_path: Vec<String>, // e.g., ["my_game", "bevy_xpbd", "cpal"]
}
