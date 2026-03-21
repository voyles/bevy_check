// audit-ignore-file
use crate::models::{AuditReport, Rule, RuleCondition, Severity, Violation};
use crate::collector::{DependencyGraph, get_dependency_chain};
use std::fs; // audit-ignore

pub fn run_audit(graph: &DependencyGraph, _target: crate::models::TargetPlatform) -> Result<AuditReport, Box<dyn std::error::Error>> {
    let mut report = AuditReport {
        portability_score: 100,
        violations: Vec::new(),
    };

    // 1. Load the rules from the JSON file
    let rules_json = fs::read_to_string("rules/default_rules.json")?; //audit-ignore
    let data: serde_json::Value = serde_json::from_str(&rules_json)?;
    let rules: Vec<Rule> = serde_json::from_value(data["rules"].clone())?;

    // 2. Iterate through every crate in the dependency graph
    for (crate_name, active_features) in &graph.packages {
        for rule in &rules {
            if crate_name == &rule.crate_id {
                let is_violation = match rule.condition {
                    RuleCondition::Always => true,
                    RuleCondition::FeatureEnabled => {
                        rule.feature.as_ref().map_or(false, |f| active_features.contains(f))
                    }
                };

                if is_violation {
                    // Deduct from score based on severity
                    match rule.severity {
                        Severity::Critical => report.portability_score = 0,
                        Severity::Warning => {
                            if report.portability_score > 10 {
                                report.portability_score -= 10;
                            }
                        }
                        _ => {}
                    }

                    // Build the dependency path for the report
                    let path = get_dependency_chain(crate_name.clone(), &graph.trace_map);

                        report.violations.push(Violation {
                            crate_name: crate_name.clone(),
                            severity: match rule.severity { 
                                Severity::Critical => Severity::Critical, 
                                Severity::Warning => Severity::Warning,
                                Severity::Info => Severity::Info
                            },
                            message: rule.reason.clone(),
                            help: rule.remediation.clone(),
                            dependency_path: path,
                        });
                    }
                }
            }
        }
    
        Ok(report)
    }
