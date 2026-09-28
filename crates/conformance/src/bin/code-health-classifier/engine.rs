//! Bounded classifier protocol and whole-request orchestration.

use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Read, Write},
    path::{Component, PathBuf},
};

use serde::{Deserialize, Serialize};
use syn::visit::Visit;

use super::{
    MAX_FILES, MAX_PATH_BYTES, MAX_REQUEST_BYTES, MAX_SOURCE_BYTES, MAX_TOTAL_SOURCE_BYTES,
    graph::{ModuleCollector, ParsedFile, Reachability, inherited_test_paths},
    projection::project_lines,
    spans::SpanCollector,
};

pub(super) const PROTOCOL_VERSION: u32 = 2;
pub(super) const CLASSIFIER_NAME: &str = "syn-ast-v1";
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Request {
    pub(super) protocol_version: u32,
    pub(super) sources: Vec<SourceInput>,
    pub(super) target_roots: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SourceInput {
    pub(super) path: String,
    pub(super) source: String,
}

#[derive(Debug, Serialize)]
pub(super) struct Response {
    pub(super) classifier: &'static str,
    pub(super) files: Vec<FileOutput>,
    pub(super) inherited_inline_paths: Vec<String>,
    pub(super) protocol_version: u32,
}

#[derive(Debug, Serialize)]
pub(super) struct FileOutput {
    pub(super) inline_test_lines: Vec<usize>,
    pub(super) path: String,
}

pub(super) fn parse_source(path: &str, source: &str) -> Result<ParsedFile, String> {
    let file = syn::parse_file(source).map_err(|error| {
        let start = error.span().start();
        format!(
            "{path}:{}:{}: Rust parse failed: {error}",
            start.line,
            start.column + 1
        )
    })?;
    let mut span_collector = SpanCollector {
        source,
        spans: Vec::new(),
        error: None,
    };
    span_collector.visit_file(&file);
    if let Some(error) = span_collector.error {
        return Err(format!("{path}: {error}"));
    }
    let mut module_collector = ModuleCollector {
        context: Vec::new(),
        context_from_source_directory: false,
        inherited: Reachability::Production,
        test_edges: Vec::new(),
        production_edges: Vec::new(),
        error: None,
    };
    module_collector.inherited = module_collector.local_reachability(&file.attrs);
    module_collector.visit_items(&file.items);
    if let Some(error) = module_collector.error {
        return Err(format!("{path}: {error}"));
    }
    Ok(ParsedFile {
        spans: span_collector.spans,
        test_edges: module_collector.test_edges,
        production_edges: module_collector.production_edges,
    })
}

pub(super) fn classify(request: Request) -> Result<Response, String> {
    if request.protocol_version != PROTOCOL_VERSION {
        return Err(format!(
            "unsupported protocol version {}; expected {PROTOCOL_VERSION}",
            request.protocol_version
        ));
    }
    if request.sources.len() > MAX_FILES || request.target_roots.len() > MAX_FILES {
        return Err(format!("more than {MAX_FILES} source files"));
    }
    let mut total = 0usize;
    let mut sources = BTreeMap::new();
    for input in request.sources {
        if input.path.len() > MAX_PATH_BYTES || input.path.is_empty() {
            return Err("source path is empty or exceeds the protocol bound".to_owned());
        }
        let path = PathBuf::from(&input.path);
        if path.is_absolute()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
            || path.extension().and_then(|value| value.to_str()) != Some("rs")
        {
            return Err(format!(
                "invalid repository-relative Rust path: {}",
                input.path
            ));
        }
        if input.source.len() > MAX_SOURCE_BYTES {
            return Err(format!(
                "{} exceeds {MAX_SOURCE_BYTES} source bytes",
                input.path
            ));
        }
        total = total
            .checked_add(input.source.len())
            .ok_or_else(|| "total source byte count overflowed".to_owned())?;
        if total > MAX_TOTAL_SOURCE_BYTES {
            return Err(format!(
                "total source bytes exceed {MAX_TOTAL_SOURCE_BYTES}"
            ));
        }
        if sources.insert(path, input.source).is_some() {
            return Err(format!("duplicate source path: {}", input.path));
        }
    }

    let mut roots = BTreeSet::new();
    for raw_path in request.target_roots {
        if raw_path.len() > MAX_PATH_BYTES || raw_path.is_empty() {
            return Err("target-root path is empty or exceeds the protocol bound".to_owned());
        }
        let path = PathBuf::from(&raw_path);
        if path.is_absolute()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
            || !sources.contains_key(&path)
        {
            return Err(format!(
                "target root is not an input Rust source: {raw_path}"
            ));
        }
        if !roots.insert(path) {
            return Err(format!("duplicate target root: {raw_path}"));
        }
    }

    let mut parsed = BTreeMap::new();
    for (path, source) in &sources {
        parsed.insert(
            path.clone(),
            parse_source(&path.display().to_string(), source)?,
        );
    }
    let inherited = inherited_test_paths(&parsed, &sources, &roots)?;
    let mut files = Vec::with_capacity(sources.len());
    for (path, source) in &sources {
        let inline_test_lines = if inherited.contains(path) {
            source
                .split_inclusive('\n')
                .enumerate()
                .filter_map(|(index, line)| {
                    line.chars()
                        .any(|character| !character.is_whitespace())
                        .then_some(index + 1)
                })
                .collect()
        } else {
            project_lines(
                source,
                parsed.get(path).expect("parsed source").spans.clone(),
            )
        };
        files.push(FileOutput {
            inline_test_lines,
            path: path.display().to_string(),
        });
    }
    Ok(Response {
        classifier: CLASSIFIER_NAME,
        files,
        inherited_inline_paths: inherited
            .into_iter()
            .map(|path| path.display().to_string())
            .collect(),
        protocol_version: PROTOCOL_VERSION,
    })
}

pub(super) fn run() -> Result<(), String> {
    let mut input = Vec::new();
    io::stdin()
        .take(MAX_REQUEST_BYTES + 1)
        .read_to_end(&mut input)
        .map_err(|error| format!("could not read request: {error}"))?;
    if input.len() as u64 > MAX_REQUEST_BYTES {
        return Err(format!("request exceeds {MAX_REQUEST_BYTES} bytes"));
    }
    let request: Request =
        serde_json::from_slice(&input).map_err(|error| format!("invalid request JSON: {error}"))?;
    let response = classify(request)?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &response)
        .map_err(|error| format!("could not encode response: {error}"))?;
    output
        .write_all(b"\n")
        .map_err(|error| format!("could not write response: {error}"))?;
    Ok(())
}
