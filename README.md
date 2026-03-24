# 🎮 bevy_check

**bevy_check** is a specialized auditing tool for [Bevy Engine](https://bevyengine.org/) projects. It ensures your game remains compatible with restricted console environments (Nintendo Switch, PS5, Xbox) by analyzing your dependency tree and scanning source code for platform-specific leaks.

## 🚀 Quick Start

1. **Build the tool:**
   ```bash
   cargo build --release

2. **Audit your project:**
    Point it at any Bevy project's Cargo.toml:
    ./target/release/bevy_check --manifest-path ../my_game/Cargo.toml --target switch --scan-source

3. **Command-Line Flags**
    -m  --manifest-path Path to the Cargo.toml to audit.                            ./Cargo.toml
    -t  --target        Target console profile (switch, ps5, xbox, generic).        generic
    -s  --scan-source   Scans .rs files for std leaks (e.g., std::fs, std::net).    false

## 🔍 How It Works

1. **Dependency Audit** (collector.rs & engine.rs)
    The tool uses cargo_metadata to flatten your entire dependency tree. It identifies transitive crates and active features. If a crate (like cpal) or a specific feature (like tokio/rt-multi-thread) is flagged in the rules, the audit fails.

2. **Source Scanning** (scanner.rs)
    When --scan-source is enabled, the tool walks your src/ directory looking for "Red Flags" that cause console certification failures:

    std::fs: Use Bevy's AssetServer or SaveData APIs instead.

    std::net: Consoles require proprietary socket wrappers.

    std::process: You cannot spawn child processes or call exit() on consoles.

3. **Portability Score**
    The tool calculates a score from 0% to 100%.

    Critical Violations: Drops score to 0%.

    Warnings: Deducts 10% per occurrence.

    Exit Code: Exits with code 1 if the score is < 100, perfect for CI/CD gates.

## 📋 Adding Rules

**Customize the database in rules/default_rules.json**
    {  
      "crate_id": "crate_name",  
      "severity": "Critical | Warning | Info",  
      "condition": "always | feature_enabled",  
      "feature": "feature_name",  
      "reason": "Why this fails console certification.",  
      "remediation": "How to fix it."  
    }  
