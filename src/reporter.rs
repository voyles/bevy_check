use crate::models::{AuditReport, Severity};
use colored::*;

pub fn print_report(report: &AuditReport) {
    println!("\n{}", "=== Bevy Console Portability Report ===".bold().cyan());
    
    // Display the final score
    let score_color = if report.portability_score == 100 {
        "green"
    } else if report.portability_score > 70 {
        "yellow"
    } else {
        "red"
    };
    
    println!("Portability Score: {}", format!("{}%", report.portability_score).color(score_color).bold());
    println!("------------------------------------------");

    if report.violations.is_empty() {
        println!("{}", "✅ No portability issues found. Project is console-ready!".green());
        return;
    }

    for violation in &report.violations {
        let prefix = match violation.severity {
            Severity::Critical => "CRITICAL".on_red().white().bold(),
            Severity::Warning => "WARNING".on_yellow().black().bold(),
            Severity::Info => "INFO".on_blue().white().bold(),
        };

        println!("\n{} in crate: {}", prefix, violation.crate_name.bold());
        println!("Reason: {}", violation.message);
        println!("Fix:    {}", violation.help.italic().bright_black());
        
        // Print the "Chain of Blame"
        let path = violation.dependency_path.join(" ➔ ");
        println!("Path:   {}", path.dimmed());
    }

    println!("\n{}", "==========================================".cyan());
    if report.portability_score < 100 {
        println!("{}", "❌ Audit Failed: Please resolve critical issues before porting.".red().bold());
    }
}