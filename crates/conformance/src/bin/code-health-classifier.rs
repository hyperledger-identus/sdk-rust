//! Internal AST classifier for deterministic code-health source populations.
//!
//! This binary is repository tooling. It is not an SDK API and is never
//! published. Input and output use a bounded, versioned JSON protocol so the
//! Python report orchestrator does not parse Rust syntax.

#[path = "code-health-classifier/cfg.rs"]
mod cfg;
#[path = "code-health-classifier/engine.rs"]
mod engine;
#[path = "code-health-classifier/modules.rs"]
mod graph;
#[path = "code-health-classifier/projection.rs"]
mod projection;
#[path = "code-health-classifier/spans.rs"]
mod spans;

use engine::run;

const MAX_REQUEST_BYTES: u64 = 32 * 1024 * 1024;
const MAX_SOURCE_BYTES: usize = 2 * 1024 * 1024;
const MAX_TOTAL_SOURCE_BYTES: usize = 24 * 1024 * 1024;
const MAX_FILES: usize = 4096;
const MAX_PATH_BYTES: usize = 4096;
const MAX_SPANS_PER_FILE: usize = 262_144;
const MAX_MODULE_EDGES: usize = 131_072;

fn main() {
    if let Err(error) = run() {
        eprintln!("code-health-classifier: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
#[path = "code-health-classifier/tests.rs"]
mod tests;
