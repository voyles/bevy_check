use cargo_metadata::MetadataCommand;
use std::collections::HashMap;
use std::path::Path;

pub struct DependencyGraph {
    /// Maps crate names to their enabled features
    pub packages: HashMap<String, Vec<String>>,
    /// Maps a crate name to its parent in the dependency tree (for tracing)
    pub trace_map: HashMap<String, String>,
}

pub fn collect_dependencies(manifest_path: &Path) -> Result<DependencyGraph, Box<dyn std::error::Error>> {
    let metadata = MetadataCommand::new()
        .manifest_path(manifest_path)
        .exec()?;

    let mut package_features = HashMap::new();
    let mut trace_map = HashMap::new();

    // 1. Map Package IDs to human-readable names and features
    let mut id_to_name = HashMap::new();
    for package in &metadata.packages {
        id_to_name.insert(package.id.clone(), package.name.clone());
    }

    // 2. Process the Resolve Graph to see what is actually active
    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            let crate_name = id_to_name.get(&node.id).cloned().unwrap_or_default();
            
            // Record active features for this specific node
            package_features.insert(crate_name.clone(), node.features.clone());

            // Record child-to-parent relationships for tracing
            for dep in node.dependencies {
                if let Some(dep_name) = id_to_name.get(&dep) {
                    trace_map.insert(dep_name.clone(), crate_name.clone());
                }
            }
        }
    }

    Ok(DependencyGraph {
        packages: package_features,
        trace_map,
    })
}

/// Helper function to reconstruct the path from a problematic crate back to the root
pub fn get_dependency_chain(mut crate_name: String, trace_map: &HashMap<String, String>) -> Vec<String> {
    let mut chain = vec![crate_name.clone()];
    while let Some(parent) = trace_map.get(&crate_name) {
        chain.push(parent.clone());
        crate_name = parent.clone();
    }
    chain.reverse();
    chain
}