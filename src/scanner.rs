use crate::models::{AuditReport, Severity, Violation};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn has_file_ignore_tag(content: &str) -> bool {
    content
        .lines()
        .any(|line| line.to_lowercase().contains("audit-ignore-file"))
}

fn has_line_ignore_tag(line: &str) -> bool {
    line.to_lowercase().contains("audit-ignore")
}

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

        // Read the whole file and allow an explicit file-level ignore tag.
        let content = fs::read_to_string(path).unwrap_or_default();
        if has_file_ignore_tag(&content) {
            continue;
        }

        // Scan line-by-line and skip lines explicitly marked with 'audit-ignore'.
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || has_line_ignore_tag(trimmed) {
                continue;
            }

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