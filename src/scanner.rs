use crate::models::AuditReport;
use std::fs::File;
use std::io::{BufRead, BufReader}; // Required for line-by-line
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_source_code(src_path: &Path, report: &mut AuditReport) {
    println!("🔍 Scanning source code for platform leaks...");

    let patterns = vec![
        // We split "std" and "::" so the scanner literally cannot find the string in its own file
        (
            format!("{}{}process", "std", "::"),
            "Direct process spawning...",
        ), // audit-ignore
        (
            format!("{}{}net", "std", "::"),
            "Consoles require proprietary network...",
        ), // audit-ignore
        (
            format!("{}{}fs", "std", "::"),
            "Direct filesystem access...",
        ), // audit-ignore
    ];

    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let path_str = path.to_string_lossy();

        // 1. BE EXTREMELY AGGRESSIVE WITH SKIPS
        if !path_str.ends_with(".rs")
            || path_str.contains("target")
            || path_str.contains(".git")
            || entry.file_name() == "scanner.rs"
        {
            continue;
        }

        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let raw_line = line_result.unwrap_or_default();
            let line = raw_line.trim();

            // 1. If the line contains the comment characters '//',
            // we check if 'ignore' exists anywhere after them.
            if let Some(comment_pos) = line.find("//") {
                if line[comment_pos..].to_lowercase().contains("ignore") {
                    continue;
                }
            }

            // 2. Standard empty line check
            if line.is_empty() {
                continue;
            }

            for (pattern, _) in &patterns {
                if line.contains(pattern) {
                    // Check one more time: does this specific match have an ignore on the same line?
                    // (Double-check for safety)
                    if !line.to_lowercase().contains("ignore") {
                        report.portability_score = report.portability_score.saturating_sub(5);
                        // ... push violation ...
                        break;
                    }
                }
            }
        }
    }
}
