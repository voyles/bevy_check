// audit-ignore-file
mod collector;
mod engine;
mod models;
mod reporter;
mod scanner;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Audits Bevy projects for console compatibility")]
struct Args {
    #[arg(short, long, default_value = "./Cargo.toml")]
    manifest_path: std::path::PathBuf,

    // Change this to use the version in models
    #[arg(short, long, value_enum, default_value_t = models::TargetPlatform::Generic)]
    target: models::TargetPlatform,

    #[arg(short, long, default_value_t = false)]
    scan_source: bool,
}

fn main() {
    let args = Args::parse();

    println!("🚀 Auditing Bevy project for {} compatibility...", match args.target {
        models::TargetPlatform::Generic => "Generic Console",
        _ => "Specific Console Target",
    });

    // 1. Collect Metadata
    let crate_graph = collector::collect_dependencies(&args.manifest_path)
        .expect("Failed to collect dependency metadata");

    // 2. Run Rule Engine
    let mut report = engine::run_audit(&crate_graph, args.target)
        .expect("Failed to execute rule engine");

    // 3. Optional: Source Scanner
    if args.scan_source {
        let src_path = args.manifest_path.parent().unwrap().join("src");
        scanner::scan_source_code(&src_path, &mut report);
    }

    // 4. Report Results
    reporter::print_report(&report);

    // 5. Exit with status code for CI/CD
    if report.portability_score < 100 {
        println!("==========================================");
        println!("❌ Audit Failed: Please resolve critical issues before porting.");
        
        // This 'cfg' tells the auditor: "I know this isn't for consoles."
        #[cfg(not(target_family = "wasm"))]
        std::process::exit(1); // audit-ignore
    }
}