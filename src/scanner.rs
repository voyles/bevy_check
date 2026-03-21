use crate::models::{AuditReport, Severity, Violation};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_source_code(src_path: &Path, report: &mut AuditReport) {
    println!("🔍 Scanning source code for platform leaks...");

    let patterns = vec![
        (format!("{}{}process", "std", "::"), "Direct process spawning..."),
        (format!("{}{}net", "std", "::"), "Consoles require proprietary network..."),
        (format!("{}{}fs", "std", "::"), "Direct filesystem access..."),
    ];

    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let path_str = path.to_string_lossy();

        // Skip non-rust files, build artifacts, and the scanner itself
        if !path_str.ends_with(".rs") 
            || path_str.contains("target") 
            || path_str.contains(".git") 
            || entry.file_name() == "scanner.rs" 
        {
            continue;
        }

        // Read the whole file to check for a global 'ignore' tag
        let content = fs::read_to_string(path).unwrap_or_default();
        if content.to_lowercase().contains("ignore") {
            continue;
        }

        // Scan line-by-line if no 'ignore' tag is found
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            for (pattern, reason) in &patterns {
                if trimmed.contains(pattern) {
                    report.portability_score = report.portability_score.saturating_sub(5);

                    report.violations.push(Violation {
                        crate_name: "Local Source".to_string(),
                        severity: Severity::Warning,
                        message: format!("Found '{}': {}", pattern, reason),
                        help: "Replace with Bevy abstractions or platform-specific CFG gates.".to_string(),
                        dependency_path: vec!["src".to_string(), entry.file_name().to_string_lossy().into_owned()],
                    });
                    break; // Move to next line after first hit
                }
            }
        }
    }
}