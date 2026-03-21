use crate::models::{AuditReport, Severity, Violation};
use walkdir::WalkDir;
use std::fs;
use std::path::Path;

pub fn scan_source_code(src_path: &Path, report: &mut AuditReport) {
    println!("🔍 Scanning source code for platform leaks...");

    // Common forbidden patterns in console development
    let patterns = vec![
        ("std::process", "Direct process spawning is forbidden on most consoles."),
        ("std::net", "Consoles require proprietary network wrappers (e.g., Socket SDKs)."),
        ("std::fs", "Direct filesystem access is restricted; use Bevy AssetServer or SaveData APIs."),
        ("std::os::unix", "Unix-specific code will fail to compile on many console OSs."),
        ("std::os::windows", "Windows-specific code will fail to compile on many console OSs."),
    ];

    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        // Skip scanner.rs to avoid it flagging itself
        if entry.file_name() == "scanner.rs" {
            continue;   
        }
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            
            for (pattern, reason) in &patterns {
                if content.contains(pattern) {
                    // Reduce score for every leak found in local code
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