#!/usr/bin/env python3
"""Generate and validate deterministic sdk-rust code-health evidence."""

from __future__ import annotations

import argparse
import hashlib
import io
import json
import re
import subprocess
import sys
import tarfile
import tempfile
import tomllib
from pathlib import Path
from typing import Iterator


CONFIG_PATH = Path("docs/architecture/code-health.toml")
EXPECTED_SCHEMA = 2
ALLOWED_DISPOSITIONS = {
    "decompose",
    "deduplicate",
    "document-exception",
    "defer-with-owner",
}
CLASSIFIER_NAME = "syn-ast-v1"
CLASSIFIER_PROTOCOL_VERSION = 1
CLASSIFIER_COMMAND = (
    "cargo",
    "run",
    "--quiet",
    "--locked",
    "-p",
    "identus-conformance",
    "--bin",
    "code-health-classifier",
    "--",
)


class AuditError(RuntimeError):
    """An invalid audit input or failed analysis."""


def canonical_json(value: object) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def working_tree_sources(root: Path) -> dict[Path, str]:
    return {
        path.relative_to(root): path.read_text(encoding="utf-8")
        for path in sorted((root / "crates").glob("*/**/*.rs"))
    }


def git_tree_sources(root: Path, revision: str) -> dict[Path, str]:
    archived = subprocess.run(
        ["git", "archive", "--format=tar", revision, "--", "crates"],
        cwd=root,
        capture_output=True,
        check=True,
    ).stdout
    sources: dict[Path, str] = {}
    with tarfile.open(fileobj=io.BytesIO(archived), mode="r:") as tree:
        for member in sorted(tree.getmembers(), key=lambda item: item.name):
            if not member.isfile() or not member.name.endswith(".rs"):
                continue
            handle = tree.extractfile(member)
            if handle is None:
                raise AuditError(f"could not read archived Rust source: {member.name}")
            sources[Path(member.name)] = handle.read().decode("utf-8")
    return sources


def classify_sources(
    sources: dict[Path, str], config: dict[str, object]
) -> tuple[list[Path], list[Path], list[Path]]:
    production: list[Path] = []
    external_tests: list[Path] = []
    generated: list[Path] = []
    exclusions = {
        Path(item["path"]): item["marker"]
        for item in config.get("generated_exclusions", [])
    }
    observed_exclusions: set[Path] = set()
    for path, source in sorted(sources.items()):
        parts = path.parts
        if path in exclusions:
            marker = exclusions[path]
            header = "\n".join(source.splitlines()[:10])
            if marker not in header:
                raise AuditError(f"generated exclusion marker is absent: {path}")
            observed_exclusions.add(path)
            generated.append(path)
        elif parts[2] in {"tests", "benches"}:
            external_tests.append(path)
        elif len(parts) >= 4 and parts[2] == "src":
            production.append(path)
    missing = sorted(set(exclusions) - observed_exclusions)
    if missing:
        raise AuditError(f"generated exclusion path is absent: {missing[0]}")
    return production, external_tests, generated


def engine_version(engine: str) -> str:
    completed = subprocess.run(
        [engine, "--version"], capture_output=True, text=True, check=False, timeout=10
    )
    if completed.returncode != 0:
        raise AuditError(f"{engine} --version failed: {completed.stderr.strip()}")
    return completed.stdout.strip()


def analyze_file(root: Path, path: Path, engine: str) -> dict[str, object]:
    completed = subprocess.run(
        [engine, "-p", str(path), "-m", "-O", "json"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
        timeout=30,
    )
    if completed.returncode != 0:
        raise AuditError(f"analysis failed for {path}: {completed.stderr.strip()}")
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise AuditError(f"invalid analyzer JSON for {path}: {error}") from error


def function_spaces(space: dict[str, object]) -> Iterator[dict[str, object]]:
    if space.get("kind") == "function":
        yield space
    for child in space.get("spaces", []):
        yield from function_spaces(child)


def function_record(path: Path, item: dict[str, object], population: str) -> dict[str, object]:
    metrics = item["metrics"]
    loc = metrics["loc"]
    return {
        "cognitive": int(metrics["cognitive"]["sum"]),
        "cyclomatic": int(metrics["cyclomatic"]["sum"]),
        "end_line": int(item["end_line"]),
        "name": item["name"],
        "path": path.as_posix(),
        "population": population,
        "sloc": int(loc["sloc"]),
        "start_line": int(item["start_line"]),
    }


def load_config(root: Path) -> dict[str, object]:
    with (root / CONFIG_PATH).open("rb") as handle:
        config = tomllib.load(handle)
    expected_config_keys = {
        "schema_version",
        "contract_version",
        "engine",
        "engine_version",
        "classifier",
        "classifier_protocol_version",
        "classifier_command",
        "baseline_revision",
        "baseline_source_fingerprint_sha256",
        "baseline_report_sha256",
        "generated_exclusions",
        "attention",
        "hotspots",
    }
    if set(config) != expected_config_keys:
        raise AuditError("code-health config has unknown or missing fields")
    if (
        type(config.get("schema_version")) is not int
        or config["schema_version"] != EXPECTED_SCHEMA
    ):
        raise AuditError("unsupported code-health config schema")
    for field in ("contract_version", "engine", "engine_version", "classifier"):
        if not isinstance(config.get(field), str) or not config[field]:
            raise AuditError(f"{field} must be a non-empty string")
    if config["classifier"] != CLASSIFIER_NAME:
        raise AuditError("code-health classifier identity does not match the implementation")
    if (
        type(config.get("classifier_protocol_version")) is not int
        or config["classifier_protocol_version"] != CLASSIFIER_PROTOCOL_VERSION
    ):
        raise AuditError("code-health classifier protocol does not match the implementation")
    if config.get("classifier_command") != list(CLASSIFIER_COMMAND):
        raise AuditError("code-health classifier command must match the pinned execution path")
    attention = config.get("attention")
    expected_attention = {
        "function_sloc",
        "cognitive",
        "cyclomatic",
        "module_authored_nonblank_lines",
    }
    if not isinstance(attention, dict) or set(attention) != expected_attention:
        raise AuditError("code-health attention policy has unknown or missing fields")
    if any(type(value) is not int or value < 0 for value in attention.values()):
        raise AuditError("code-health attention values must be non-negative integers")
    for field in ("baseline_revision",):
        if not re.fullmatch(r"[0-9a-f]{40}", str(config.get(field, ""))):
            raise AuditError(f"{field} must be an exact Git SHA")
    for field in ("baseline_source_fingerprint_sha256", "baseline_report_sha256"):
        if not re.fullmatch(r"[0-9a-f]{64}", str(config.get(field, ""))):
            raise AuditError(f"{field} must be a SHA-256 digest")
    generated_config = config.get("generated_exclusions", [])
    if not isinstance(generated_config, list):
        raise AuditError("generated_exclusions must be a list")
    generated_paths: set[str] = set()
    for exclusion in generated_config:
        if not isinstance(exclusion, dict) or set(exclusion) != {"path", "marker"}:
            raise AuditError("generated exclusions require only path and marker")
        path = exclusion["path"]
        marker = exclusion["marker"]
        if (
            not isinstance(path, str)
            or not re.fullmatch(r"crates/[^/]+/(?:src|tests|benches)/.+\.rs", path)
            or path in generated_paths
            or not isinstance(marker, str)
            or not marker
            or "\n" in marker
        ):
            raise AuditError("generated exclusion must have one exact unique Rust path and marker")
        generated_paths.add(path)
    seen: set[str] = set()
    for hotspot in config.get("hotspots", []):
        if not isinstance(hotspot, dict) or set(hotspot) != {
            "id",
            "locations",
            "disposition",
            "owner",
            "evidence",
        }:
            raise AuditError("hotspot has unknown or missing fields")
        identifier = hotspot.get("id")
        if not isinstance(identifier, str) or not identifier or identifier in seen:
            raise AuditError("hotspot ids must be unique non-empty strings")
        seen.add(identifier)
        if hotspot.get("disposition") not in ALLOWED_DISPOSITIONS:
            raise AuditError(f"hotspot {identifier} has invalid disposition")
        locations = hotspot.get("locations")
        if (
            not isinstance(hotspot.get("owner"), str)
            or not hotspot["owner"]
            or not isinstance(hotspot.get("evidence"), str)
            or not hotspot["evidence"]
            or not isinstance(locations, list)
            or not locations
            or any(not isinstance(location, str) or not location for location in locations)
        ):
            raise AuditError(f"hotspot {identifier} lacks owner, evidence, or locations")
    if not seen:
        raise AuditError("at least one hotspot classification is required")
    return config


def source_fingerprint(sources: dict[Path, str], paths: list[Path]) -> str:
    digest = hashlib.sha256()
    for path in sorted(paths):
        digest.update(path.as_posix().encode())
        digest.update(b"\0")
        digest.update(sources[path].encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()


def resolve_revision(root: Path, requested: str | None) -> str:
    candidate = requested or "HEAD"
    resolved = subprocess.run(
        ["git", "rev-parse", "--verify", f"{candidate}^{{commit}}"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()
    if not re.fullmatch(r"[0-9a-f]{40}", resolved):
        raise AuditError(f"Git did not resolve an exact commit for {candidate!r}")
    return resolved


def exact_working_revision(root: Path, requested: str | None) -> str:
    resolved = resolve_revision(root, requested)
    changed = subprocess.run(
        ["git", "diff", "--name-only", resolved, "--", "crates"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    changed_rust = [path for path in changed.stdout.splitlines() if path.endswith(".rs")]
    untracked = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "--", "crates"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    untracked_rust = [path for path in untracked if path.endswith(".rs")]
    if changed.returncode != 0 or changed_rust or untracked_rust:
        raise AuditError(
            "authored Rust does not match the requested revision; commit it or audit an exact clean worktree"
        )
    return resolved


def rust_classifier_population(
    root: Path,
    sources: dict[Path, str],
    command: tuple[str, ...] = CLASSIFIER_COMMAND,
) -> tuple[dict[Path, set[int]], set[Path]]:
    request = {
        "protocol_version": CLASSIFIER_PROTOCOL_VERSION,
        "sources": [
            {"path": path.as_posix(), "source": source}
            for path, source in sorted(sources.items())
        ],
    }
    completed = subprocess.run(
        command,
        cwd=root,
        input=json.dumps(request, ensure_ascii=False, separators=(",", ":")),
        capture_output=True,
        text=True,
        check=False,
        timeout=180,
    )
    if completed.returncode != 0:
        diagnostic = completed.stderr.strip() or "classifier exited without a diagnostic"
        raise AuditError(f"Rust code-health classifier failed: {diagnostic}")
    try:
        response = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise AuditError(f"Rust code-health classifier returned invalid JSON: {error}") from error
    response = require_exact_keys(
        response,
        {"classifier", "files", "inherited_inline_paths", "protocol_version"},
        "classifier response",
    )
    if response["classifier"] != CLASSIFIER_NAME:
        raise AuditError("Rust code-health classifier identity mismatch")
    if (
        type(response["protocol_version"]) is not int
        or response["protocol_version"] != CLASSIFIER_PROTOCOL_VERSION
    ):
        raise AuditError("Rust code-health classifier protocol mismatch")
    files = response["files"]
    if not isinstance(files, list) or len(files) != len(sources):
        raise AuditError("Rust code-health classifier file coverage mismatch")
    lines_by_path: dict[Path, set[int]] = {}
    for entry in files:
        entry = require_exact_keys(entry, {"path", "inline_test_lines"}, "classifier file")
        if not isinstance(entry["path"], str):
            raise AuditError("Rust code-health classifier returned a non-string path")
        path = Path(entry["path"])
        lines = entry["inline_test_lines"]
        if path not in sources or path in lines_by_path:
            raise AuditError(f"Rust code-health classifier returned unexpected path: {path}")
        if (
            not isinstance(lines, list)
            or any(type(line) is not int or line <= 0 for line in lines)
            or lines != sorted(set(lines))
            or any(line > len(sources[path].splitlines()) for line in lines)
        ):
            raise AuditError(f"Rust code-health classifier returned invalid lines: {path}")
        lines_by_path[path] = set(lines)
    inherited_raw = response["inherited_inline_paths"]
    if not isinstance(inherited_raw, list) or any(
        not isinstance(path, str) for path in inherited_raw
    ):
        raise AuditError("Rust code-health classifier returned invalid inherited paths")
    inherited = {Path(path) for path in inherited_raw}
    if not inherited.issubset(sources) or len(inherited) != len(inherited_raw):
        raise AuditError("Rust code-health classifier inherited-path coverage mismatch")
    return lines_by_path, inherited


def source_population_evidence(
    root: Path, sources: dict[Path, str], config: dict[str, object]
) -> tuple[
    list[Path],
    list[Path],
    list[Path],
    dict[Path, set[int]],
    dict[str, dict[str, int]],
]:
    production_paths, test_paths, generated_paths = classify_sources(sources, config)
    classifier_paths = sorted([*production_paths, *generated_paths])
    classifier_sources = {path: sources[path] for path in classifier_paths}
    all_inline_lines, _ = rust_classifier_population(
        root, classifier_sources, tuple(config["classifier_command"])
    )
    inline_lines_by_path: dict[Path, set[int]] = {}
    counts = {
        "production": {"authored_nonblank_lines": 0, "files": 0, "functions": 0},
        "external_test": {"authored_nonblank_lines": 0, "files": 0, "functions": 0},
        "inline_test": {"authored_nonblank_lines": 0, "files": 0, "functions": 0},
    }
    for path in production_paths:
        source = sources[path]
        lines = source.splitlines()
        inline_lines = all_inline_lines[path]
        inline_lines_by_path[path] = inline_lines
        production_lines = sum(
            1
            for number, line in enumerate(lines, 1)
            if line.strip() and number not in inline_lines
        )
        test_lines = sum(
            1
            for number, line in enumerate(lines, 1)
            if line.strip() and number in inline_lines
        )
        if production_lines:
            counts["production"]["files"] += 1
            counts["production"]["authored_nonblank_lines"] += production_lines
        if test_lines:
            counts["inline_test"]["files"] += 1
            counts["inline_test"]["authored_nonblank_lines"] += test_lines
    for path in test_paths:
        counts["external_test"]["files"] += 1
        counts["external_test"]["authored_nonblank_lines"] += sum(
            1 for line in sources[path].splitlines() if line.strip()
        )
    return (
        production_paths,
        test_paths,
        generated_paths,
        inline_lines_by_path,
        counts,
    )


def population_projection_sha256(
    production_paths: list[Path],
    test_paths: list[Path],
    generated_paths: list[Path],
    inline_lines_by_path: dict[Path, set[int]],
) -> str:
    projection = {
        "external_test_paths": [path.as_posix() for path in test_paths],
        "generated_paths": [path.as_posix() for path in generated_paths],
        "production_sources": [
            {
                "inline_test_lines": sorted(inline_lines_by_path[path]),
                "path": path.as_posix(),
            }
            for path in production_paths
        ],
    }
    return hashlib.sha256(canonical_json(projection).encode("utf-8")).hexdigest()


def build_report(
    repository_root: Path,
    analysis_root: Path,
    sources: dict[Path, str],
    config: dict[str, object],
    revision: str,
    engine: str,
) -> dict[str, object]:
    (
        production_paths,
        test_paths,
        generated_paths,
        inline_lines_by_path,
        counts,
    ) = source_population_evidence(repository_root, sources, config)
    all_authored = production_paths + test_paths
    functions: list[dict[str, object]] = []
    modules: list[dict[str, object]] = []

    for path in production_paths:
        source = sources[path]
        inline_lines = inline_lines_by_path[path]
        lines = source.splitlines()
        production_lines = sum(1 for number, line in enumerate(lines, 1) if line.strip() and number not in inline_lines)
        if production_lines:
            modules.append({"authored_nonblank_lines": production_lines, "path": path.as_posix()})

        result = analyze_file(analysis_root, path, engine)
        for item in function_spaces(result):
            line = int(item["start_line"])
            population = "inline_test" if line in inline_lines else "production"
            counts[population]["functions"] += 1
            functions.append(function_record(path, item, population))

    for path in test_paths:
        result = analyze_file(analysis_root, path, engine)
        for item in function_spaces(result):
            counts["external_test"]["functions"] += 1
            functions.append(function_record(path, item, "external_test"))

    attention = config["attention"]
    function_signals = [
        item
        for item in functions
        if item["population"] == "production"
        and (
            item["sloc"] > attention["function_sloc"]
            or item["cognitive"] > attention["cognitive"]
            or item["cyclomatic"] > attention["cyclomatic"]
        )
    ]
    function_signals.sort(key=lambda item: (item["path"], item["start_line"], item["name"]))
    module_signals = [
        item
        for item in modules
        if item["authored_nonblank_lines"] > attention["module_authored_nonblank_lines"]
    ]
    module_signals.sort(key=lambda item: item["path"])

    return {
        "analyzer": {
            "classifier": config["classifier"],
            "classifier_command": config["classifier_command"],
            "classifier_protocol_version": config["classifier_protocol_version"],
            "contract_version": config["contract_version"],
            "engine": engine,
            "engine_version": config["engine_version"],
        },
        "attention": attention,
        "generated_exclusions": [path.as_posix() for path in generated_paths],
        "hotspots": config["hotspots"],
        "population_projection_sha256": population_projection_sha256(
            production_paths,
            test_paths,
            generated_paths,
            inline_lines_by_path,
        ),
        "populations": counts,
        "revision": revision,
        "schema_version": EXPECTED_SCHEMA,
        "signals": {
            "functions": function_signals,
            "modules": module_signals,
        },
        "source_fingerprint_sha256": source_fingerprint(sources, all_authored),
    }


def checked_engine(config: dict[str, object]) -> str:
    engine = str(config["engine"])
    expected = f"{engine} {config['engine_version']}"
    actual = engine_version(engine)
    if actual != expected:
        raise AuditError(f"expected exactly {expected}, found {actual}")
    return engine


def audit(root: Path, revision: str | None) -> dict[str, object]:
    config = load_config(root)
    engine = checked_engine(config)
    resolved = exact_working_revision(root, revision)
    sources = working_tree_sources(root)
    return build_report(root, root, sources, config, resolved, engine)


def regenerate_baseline(root: Path) -> dict[str, object]:
    config = load_config(root)
    engine = checked_engine(config)
    revision = resolve_revision(root, str(config["baseline_revision"]))
    sources = git_tree_sources(root, revision)
    with tempfile.TemporaryDirectory(prefix="sdk-rust-code-health-") as directory:
        analysis_root = Path(directory)
        for path, source in sources.items():
            target = analysis_root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(source, encoding="utf-8")
        return build_report(root, analysis_root, sources, config, revision, engine)


def require_exact_keys(value: object, expected: set[str], context: str) -> dict[str, object]:
    if not isinstance(value, dict) or set(value) != expected:
        raise AuditError(f"{context} has unknown or missing fields")
    return value


def require_nonnegative_int(value: object, context: str) -> None:
    if type(value) is not int or value < 0:
        raise AuditError(f"{context} must be a non-negative integer")


def validate_report(root: Path, path: Path, policy_only: bool = False) -> dict[str, object]:
    raw = path.read_text(encoding="utf-8")
    try:
        report = json.loads(raw)
    except json.JSONDecodeError as error:
        raise AuditError(f"invalid report JSON: {error}") from error
    if raw != canonical_json(report):
        raise AuditError("report is not canonical sorted/indented JSON")
    config = load_config(root)
    if hashlib.sha256(raw.encode("utf-8")).hexdigest() != config["baseline_report_sha256"]:
        raise AuditError("report content does not match the policy-pinned digest")
    report = require_exact_keys(
        report,
        {
            "analyzer",
            "attention",
            "generated_exclusions",
            "hotspots",
            "population_projection_sha256",
            "populations",
            "revision",
            "schema_version",
            "signals",
            "source_fingerprint_sha256",
        },
        "report",
    )
    if (
        type(report.get("schema_version")) is not int
        or report["schema_version"] != EXPECTED_SCHEMA
    ):
        raise AuditError("report schema version mismatch")
    analyzer = require_exact_keys(
        report["analyzer"],
        {
            "classifier",
            "classifier_command",
            "classifier_protocol_version",
            "contract_version",
            "engine",
            "engine_version",
        },
        "analyzer",
    )
    for field in ("classifier", "contract_version", "engine", "engine_version"):
        if not isinstance(analyzer.get(field), str) or not analyzer[field]:
            raise AuditError(f"report analyzer {field} must be a non-empty string")
        if analyzer.get(field) != config.get(field):
            raise AuditError(f"report analyzer {field} does not match config")
    if (
        type(analyzer.get("classifier_protocol_version")) is not int
        or analyzer["classifier_protocol_version"]
        != config.get("classifier_protocol_version")
    ):
        raise AuditError("report classifier protocol does not match config")
    if analyzer.get("classifier_command") != config.get("classifier_command"):
        raise AuditError("report classifier command does not match config")
    attention = require_exact_keys(
        report["attention"],
        {"function_sloc", "cognitive", "cyclomatic", "module_authored_nonblank_lines"},
        "attention",
    )
    if attention != config.get("attention"):
        raise AuditError("report attention policy does not match config")
    for field, value in attention.items():
        require_nonnegative_int(value, f"attention {field}")
    if not re.fullmatch(r"[0-9a-f]{40}", str(report.get("revision", ""))):
        raise AuditError("report revision must be an exact 40-character Git SHA")
    if report["revision"] != config["baseline_revision"]:
        raise AuditError("report revision does not match pinned baseline policy")
    if not re.fullmatch(r"[0-9a-f]{64}", str(report.get("source_fingerprint_sha256", ""))):
        raise AuditError("report source fingerprint must be SHA-256")
    if report["source_fingerprint_sha256"] != config["baseline_source_fingerprint_sha256"]:
        raise AuditError("report source fingerprint does not match pinned baseline policy")
    if not re.fullmatch(
        r"[0-9a-f]{64}", str(report.get("population_projection_sha256", ""))
    ):
        raise AuditError("report population projection must be SHA-256")
    if report.get("hotspots") != config.get("hotspots"):
        raise AuditError("report hotspot classifications are stale")
    populations = require_exact_keys(
        report["populations"], {"production", "external_test", "inline_test"}, "populations"
    )
    for population in ("production", "external_test", "inline_test"):
        values = require_exact_keys(
            populations[population],
            {"files", "functions", "authored_nonblank_lines"},
            f"{population} population",
        )
        for field, value in values.items():
            require_nonnegative_int(value, f"{population} {field}")
    if not isinstance(report.get("generated_exclusions"), list):
        raise AuditError("generated exclusions must be a list")
    expected_generated = [item["path"] for item in config.get("generated_exclusions", [])]
    if report["generated_exclusions"] != expected_generated:
        raise AuditError("generated exclusions do not match the exact policy allowlist")
    signals = require_exact_keys(report["signals"], {"functions", "modules"}, "signals")
    if not isinstance(signals.get("functions"), list) or not isinstance(signals.get("modules"), list):
        raise AuditError("report signals must contain function and module lists")
    for item in signals["functions"]:
        item = require_exact_keys(
            item,
            {
                "cognitive",
                "cyclomatic",
                "end_line",
                "name",
                "path",
                "population",
                "sloc",
                "start_line",
            },
            "function signal",
        )
        for field in ("cognitive", "cyclomatic", "end_line", "sloc", "start_line"):
            require_nonnegative_int(item[field], f"function signal {field}")
        if not isinstance(item["name"], str) or not isinstance(item["path"], str):
            raise AuditError("function signal name and path must be strings")
        if item["population"] != "production":
            raise AuditError("attention function signals must be production")
    for item in signals["modules"]:
        item = require_exact_keys(
            item, {"authored_nonblank_lines", "path"}, "module signal"
        )
        require_nonnegative_int(item["authored_nonblank_lines"], "module signal lines")
        if not isinstance(item["path"], str):
            raise AuditError("module signal path must be a string")

    if not policy_only:
        sources = git_tree_sources(root, report["revision"])
        (
            production_paths,
            test_paths,
            generated_paths,
            inline_lines_by_path,
            source_counts,
        ) = source_population_evidence(root, sources, config)
        fingerprint = source_fingerprint(sources, production_paths + test_paths)
        if fingerprint != report["source_fingerprint_sha256"]:
            raise AuditError("report fingerprint does not match its pinned Git tree")
        if [path.as_posix() for path in generated_paths] != report["generated_exclusions"]:
            raise AuditError("report generated exclusions do not match its pinned Git tree")
        projection = population_projection_sha256(
            production_paths,
            test_paths,
            generated_paths,
            inline_lines_by_path,
        )
        if projection != report["population_projection_sha256"]:
            raise AuditError(
                "report population projection does not match its pinned Git tree"
            )
        for population in ("production", "external_test", "inline_test"):
            for field in ("files", "authored_nonblank_lines"):
                if source_counts[population][field] != populations[population][field]:
                    raise AuditError(
                        f"report {population} {field} does not match its pinned Git tree"
                    )
    return report


def verify_baseline(root: Path, path: Path) -> None:
    report = validate_report(root, path)
    regenerated = regenerate_baseline(root)
    if report != regenerated:
        raise AuditError("pinned baseline does not match regenerated Git-tree analysis")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--output", type=Path)
    parser.add_argument("--revision")
    parser.add_argument("--policy-only", action="store_true")
    action = parser.add_mutually_exclusive_group()
    action.add_argument("--check-report", type=Path)
    action.add_argument("--verify-baseline", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        if args.check_report:
            validate_report(root, args.check_report, policy_only=args.policy_only)
            print(f"code-health: report contract passed: {args.check_report}")
            return 0
        if args.policy_only:
            raise AuditError("--policy-only requires --check-report")
        if args.verify_baseline:
            baseline = root / "docs/architecture/code-health-baseline.json"
            verify_baseline(root, baseline)
            print(f"code-health: pinned baseline regenerated exactly: {baseline}")
            return 0
        report = audit(root, args.revision)
        rendered = canonical_json(report)
        if args.output:
            args.output.write_text(rendered, encoding="utf-8")
            print(f"code-health: wrote {args.output}")
        else:
            sys.stdout.write(rendered)
        return 0
    except (AuditError, OSError, subprocess.SubprocessError) as error:
        print(f"code-health: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
