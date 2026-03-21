use crate::models::{AuditReport, Severity, Violation};
use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufRead, BufReader}; // Required for line-by-line
use std::path::Path;

pub fn scan_source_code(src_path: &Path, report: &mut AuditReport) {
    println!("🔍 Scanning source code for platform leaks...");

    let patterns = vec![
    (   format!("std::{}", "process"), "Direct process spawning is forbidden."), // audit-ignore
        (format!("std::{}", "net"), "Consoles require proprietary network wrappers."), // audit-ignore
        (format!("std::{}", "fs"), "Direct filesystem access is restricted."), // audit-ignore
    ];

    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        let path_str = entry.path();
        let path_str = path_str.to_string_lossy();

        if !path_str.ends_with(".rs") {
            continue;
        }
        
        // Skip scanner.rs to avoid self-flagging
        if entry.file_name() == "scanner.rs" || path_str.contains("target") || path_str.contains(".git") {
            continue;   
        }

        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let file = File::open(entry.path()).expect("Failed to open file");
            let reader = BufReader::new(file);
            
            for line_result in reader.lines() {
                let line = line_result.unwrap_or_default();

                // Skip scanning this line if the developer vetted it
                if line.contains("audit-ignore") {
                    continue;
                }

                for (pattern, reason) in &patterns {
                    if line.contains(pattern) {
                        if report.portability_score > 5 {
                            report.portability_score -= 5;
                        }

                        report.violations.push(Violation {
                            crate_name: "Local Source".to_string(),
                            severity: Severity::Warning,
                            message: format!("Found '{}' in {:?}: {}", pattern, entry.file_name(), reason),
                            help: "Replace with Bevy abstractions or platform-specific CFG gates.".to_string(),
                            dependency_path: vec!["src".to_string(), entry.file_name().to_string_lossy().into_owned()],
                        });
                    }
                }
            }
        }
    }
}