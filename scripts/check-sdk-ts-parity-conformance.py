#!/usr/bin/env python3
"""Validate the SDK-Rust/SDK-TS parity and conformance milestone."""

from __future__ import annotations

import re
import sys
import tomllib
from collections import Counter
from pathlib import Path


MATRIX_PATH = Path("docs/roadmap/sdk-ts-parity-conformance.toml")
INVENTORY_PATH = Path("docs/architecture/identus-platform-ts-capabilities.toml")
REPORT_PATH = Path("docs/roadmap/sdk-ts-parity-conformance.md")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
RELATIONS = {
    "semantic-equivalent",
    "compatible-adapter",
    "platform-owned",
    "intentionally-different",
    "deferred",
    "retired",
}
STATES = {"delivered", "partial", "blocked", "excluded", "not-applicable"}
DIMENSIONS = {
    "normative-profile",
    "rust-contract",
    "adapter-mapping",
    "shared-vectors",
    "error-behavior",
    "resource-security",
    "target-runtime",
    "consumer-proof",
    "rollback",
}
TOP_LEVEL_KEYS = {
    "schema_version",
    "milestone_id",
    "milestone_number",
    "milestone_url",
    "title",
    "program_issue",
    "definition_issue",
    "status",
    "snapshot_date",
    "source",
    "delivery",
    "vocabulary",
    "ci",
    "target",
    "success_criterion",
    "capability",
}
CAPABILITY_KEYS = {
    "id",
    "relation",
    "state",
    "rationale",
    "required_dimensions",
    "known_deviations",
    "evidence",
    "owner_issue",
    "next_action",
    "consumer",
    "rollback",
    "non_claims",
    "rust_surface",
    "target_surface",
    "applicable_targets",
    "target_states",
    "missing_dimensions",
    "missing_proof",
}
TARGETS = ("browser", "nodejs", "react-web", "react-native")
TARGET_KEYS = {
    "id",
    "state",
    "runtime_matrix",
    "binding",
    "artifact",
    "module_format",
    "loader",
    "host_ports",
    "consumer_fixture",
    "support_tier",
    "ci_job",
    "success_evidence",
    "limitations",
    "non_claims",
}
SUCCESS_CRITERION_KEYS = {
    "id",
    "title",
    "pass_condition",
    "evidence",
    "applicability",
    "not_applicable_policy",
    "non_claims",
}
SUCCESS_CRITERIA = tuple(f"SC-{number:02d}" for number in range(1, 10))
SUCCESS_CRITERIA_CONTRACT = {
    "SC-01": (
        "Capability-oriented TypeScript contract",
        "New public exports, package paths, and primary documentation use capability or standards names and contain none of the closed legacy-public-name vocabulary.",
        ("public API and package-export snapshot", "case-insensitive legacy-name guard", "migration-only exception inventory"),
        "all new Rust-backed TypeScript public surfaces",
        "No exemption for new public names; donor-history and private legacy providers are outside the public scan.",
        ("identical Rust and TypeScript API shapes", "removal of private legacy providers"),
    ),
    "SC-02": (
        "Target-specific build and runtime proof",
        "Every claimed target has delivered build, artifact, loader, package, consumer E2E, resource, budget, and rollback evidence for its pinned runtime matrix.",
        ("target record", "exact artifact identity", "consumer fixture", "accepted target budget", "rollback rehearsal"),
        "each applicable target of a delivered capability",
        "Requires owner, rationale, review or expiry trigger, and reconsideration rule in the capability record.",
        ("cross-target inference", "support from compile-only evidence"),
    ),
    "SC-03": (
        "Applicable SDK-TS E2E reuse",
        "Every discovered donor E2E test is ledgered and each shared journey runs one TypeScript assertion body against legacy and Rust-backed private providers on every applicable target.",
        ("donor revision path suite test ID and content hash", "discovered-count equality", "disposition and authority", "no unledgered skip focus or quarantine"),
        "all SDK-TS E2E tests discovered from the pinned donor revision",
        "Only adapted, platform-owned, superseded, or excluded with owner, rationale, replacement evidence, and reconsideration trigger.",
        ("all donor behavior is normative", "unchanged execution for superseded profiles"),
    ),
    "SC-04": (
        "Rust evidence for additions and deviations",
        "Additional or intentionally different portable behavior has Rust-owned normative, negative, resource-bound, and E2E or conformance evidence plus applicable quality-evidence dispositions.",
        ("rust-addition or rust-deviation ledger", "selected profile", "negative and boundary tests", "TypeScript boundary smoke"),
        "every Rust addition or deviation from the donor journey",
        "Requires a stable quality-evidence declaration or owner, rationale, review trigger, and reconsideration rule per evidence class.",
        ("TypeScript regressions define Rust semantics", "coverage alone establishes authority"),
    ),
    "SC-05": (
        "Coverage and public-operation exercise",
        "Every owned public operation maps to contract or E2E evidence; owned changed executable code reaches at least 80 percent line and 70 percent branch coverage where reliable and never regresses below a higher established package baseline.",
        ("public-operation inventory", "pinned tool version", "base head numerator denominator target and profile", "coverage artifact", "condition mutation and negative evidence where branch coverage is unavailable"),
        "owned changed executable Rust and TypeScript adapter code",
        "Unsupported measures require an owner, tool limitation, strongest alternative evidence, and reconsideration trigger; invented percentages are forbidden.",
        ("coverage proves conformance", "lowering an established higher baseline"),
    ),
    "SC-06": (
        "Type-safe and validated boundary",
        "Pinned strict TypeScript declarations expose no explicit any, narrow unknown at ingress, validate wire values, and map numbers, bigint, bytes, null, undefined, errors, lifecycle, Promises, cancellation, and AbortSignal deterministically while Rust revalidates domain invariants.",
        ("strict compiler receipt", "public declaration snapshot", "runtime wire-validation tests", "DTO error and API drift snapshots", "lifecycle and cancellation tests"),
        "all owned TypeScript binding and facade surfaces",
        "Platform-owned values still require strict host types and a bounded mapping rationale.",
        ("unknown is forbidden at ingress", "TypeScript validation replaces Rust validation"),
    ),
    "SC-07": (
        "Shared portable semantics and thin adapters",
        "Portable core crates contain no inward binding or platform dependency; bindings depend inward only and TypeScript duplicates no cryptography, canonical parsing, format, or protocol-state logic.",
        ("dependency-direction guard", "semantic-duplication review", "generated mapping drift test", "host-port contract tests"),
        "all Rust-backed capabilities",
        "Generated outer allocation bounds and explicit host policy are permitted when Rust remains authoritative.",
        ("one universal artifact", "platform-owned behavior moves to Rust"),
    ),
    "SC-08": (
        "Operable target with accepted budgets",
        "Before support, each target has accepted size, startup, latency, throughput, and memory budgets as applicable plus bounded diagnostics, observability, fallback, and rollback.",
        ("pinned runner and protocol", "compressed and uncompressed artifact size", "cold and warm p50 p95", "representative latency throughput and memory", "variance regression and rollback rules"),
        "each target support claim",
        "A measure may be omitted only with owner, rationale, review or expiry trigger, and reconsideration rule; until a budget is accepted results remain measurement-only.",
        ("universal performance threshold", "measurement activates support"),
    ),
    "SC-09": (
        "Rust-core-first isolated CI",
        "The independent Rust-core job passes first, a fail-closed exact-head impact plan selects all dependent target jobs, selected jobs do not cancel siblings, and the existing protected fast aggregator is satisfied only when core and every selected target succeeds.",
        ("core package-set guard", "API and schema fingerprints", "impact-plan receipt", "isolated target receipts", "single fast aggregation receipt"),
        "every pull request after CI implementation",
        "Unselected targets emit an inapplicable receipt without installing target tools; unknown impact selects all targets.",
        ("changing SDK-LIM-009", "a target failure erases core evidence", "path-only impact routing"),
    ),
}
LEGACY_PUBLIC_NAMES = {"apollo", "castor", "mercury", "pluto", "pollux", "edge-agent"}
DONOR_TEST_DISPOSITIONS = {"shared", "adapted", "platform-owned", "superseded", "excluded"}
RUST_EVIDENCE_DISPOSITIONS = {"rust-addition", "rust-deviation"}
CI = {
    "core_job": "core-rust",
    "protected_status": "fast",
    "policy_constraint": "SDK-LIM-009",
    "impact_job": "binding-impact-plan",
    "target_jobs": [
        "target-browser",
        "target-nodejs",
        "target-react-web",
        "target-react-native",
        "target-package-types",
    ],
    "impact_inputs": [
        "exported-api-fingerprint",
        "binding-schema-fingerprint",
        "transitive-bound-crates",
        "relevant-lockfiles",
        "generated-mappings",
        "package-metadata",
        "target-contracts",
        "changed-paths",
    ],
    "unknown_impact": "select-all-target-jobs",
    "selected_failure": "preserve-core-result-and-fail-fast-aggregator",
    "selected_terminal": "only-success-satisfies-selected-job; failure-cancelled-skipped-timed-out-or-missing-blocks",
    "unselected_target": "machine-readable-inapplicable-receipt-without-target-toolchain",
    "fail_fast": False,
    "dependency_direction": "bindings-and-platform-adapters-depend-inward-on-portable-core-only",
}
DELIVERY = {
    "mode": "local-first-capability-trains",
    "target_changed_loc_min": 5000,
    "target_changed_loc_max": 10000,
    "target_is_merge_gate": False,
    "excluded_loc": ["generated", "vendored", "lockfiles", "evidence-artifacts"],
    "required_contents": ["implementation", "adapters", "vectors", "migration", "documentation", "tests"],
    "split_conditions": [
        "independent-ownership",
        "security-or-release-authority",
        "independent-rollback",
        "unsafe-review-size",
    ],
    "checkpoint_policy": "signed-reviewable-commits-with-targeted-local-gates",
    "candidate_policy": "complete-applicable-local-gate-before-push",
    "hosted_ci_policy": "mandatory-final-confirmation",
}


def text(value: object, field: str, errors: list[str]) -> str:
    if not isinstance(value, str) or not value.strip():
        errors.append(f"{field} must be non-empty text")
        return ""
    return value


def strings(value: object, field: str, errors: list[str], *, allow_empty: bool = False) -> list[str]:
    if not isinstance(value, list) or (not allow_empty and not value):
        errors.append(f"{field} must be {'a' if allow_empty else 'a non-empty'} string array")
        return []
    if not all(isinstance(item, str) and item.strip() for item in value):
        errors.append(f"{field} must contain only non-empty strings")
        return []
    return value


def load(path: Path, label: str, errors: list[str]) -> dict[str, object]:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot load {label}: {error}")
        return {}


def report_group_rows(report: str, values: set[str], label: str, errors: list[str]) -> dict[str, tuple[int, set[str]]]:
    observed: dict[str, tuple[int, set[str]]] = {}
    for line in report.splitlines():
        match = re.fullmatch(r"\| `([^`]+)` \| ([0-9]+) \| (.+) \|", line)
        if not match or match.group(1) not in values:
            continue
        key, count_text, members_text = match.groups()
        members = set(re.findall(r"`([^`]+)`", members_text)) if members_text != "none" else set()
        if key in observed:
            errors.append(f"report {label} row {key} is duplicated")
        observed[key] = (int(count_text), members)
    if set(observed) != values:
        errors.append(f"report {label} rows differ: missing={sorted(values - set(observed))}")
    return observed


def table_rows_between_markers(
    report: str,
    marker: str,
    expected_header: list[str],
    errors: list[str],
) -> list[list[str]]:
    start = f"<!-- {marker}:start -->"
    end = f"<!-- {marker}:end -->"
    if report.count(start) != 1 or report.count(end) != 1:
        errors.append(f"report must contain one {marker} marker pair")
        return []
    section = report.split(start, 1)[1].split(end, 1)[0]
    parsed: list[list[str]] = []
    for line in section.splitlines():
        if not line.startswith("|") or not line.endswith("|"):
            continue
        cells = [cell.strip() for cell in line[1:-1].split("|")]
        parsed.append(cells)
    if len(parsed) < 2:
        errors.append(f"report {marker} must contain a Markdown table")
        return []
    if parsed[0] != expected_header:
        errors.append(f"report {marker} header must equal {expected_header}")
    if len(parsed[1]) != len(expected_header) or not all(
        cell and set(cell) <= {"-", ":"} for cell in parsed[1]
    ):
        errors.append(f"report {marker} separator must match its header width")
    return parsed[2:]


def unquote_code(value: str) -> str:
    return value[1:-1] if len(value) >= 2 and value.startswith("`") and value.endswith("`") else value


def expected_target_summary(target_states: dict[str, object]) -> str:
    return "; ".join(f"{target}={target_states.get(target)}" for target in TARGETS)


def expected_gap_summary(row: dict[str, object]) -> str:
    deviations = row.get("known_deviations")
    prefix = "; ".join(deviations) + "; " if isinstance(deviations, list) and deviations else ""
    return f"{prefix}Missing proof: {row.get('missing_proof')}"


def joined(value: object) -> str:
    return "; ".join(str(item) for item in value) if isinstance(value, list) else str(value)


def expected_target_report_row(row: dict[str, object]) -> list[str]:
    return [
        str(row.get("id")),
        str(row.get("state")),
        joined(row.get("runtime_matrix")),
        str(row.get("binding")),
        str(row.get("artifact")),
        str(row.get("module_format")),
        str(row.get("loader")),
        joined(row.get("host_ports")),
        str(row.get("consumer_fixture")),
        str(row.get("support_tier")),
        str(row.get("ci_job")),
        joined(row.get("success_evidence")),
        f"limitations: {joined(row.get('limitations'))}; non-claims: {joined(row.get('non_claims'))}",
    ]


def expected_success_report_row(row: dict[str, object]) -> list[str]:
    return [
        str(row.get("id")),
        str(row.get("title")),
        str(row.get("pass_condition")),
        joined(row.get("evidence")),
        str(row.get("applicability")),
        str(row.get("not_applicable_policy")),
        joined(row.get("non_claims")),
    ]


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
    errors: list[str] = []
    matrix = load(root / MATRIX_PATH, str(MATRIX_PATH), errors)
    inventory = load(root / INVENTORY_PATH, str(INVENTORY_PATH), errors)
    try:
        report = (root / REPORT_PATH).read_text(encoding="utf-8")
    except OSError as error:
        errors.append(f"cannot load {REPORT_PATH}: {error}")
        report = ""
    if errors:
        print("sdk-ts-parity-conformance: " + "; ".join(errors), file=sys.stderr)
        return 1

    if set(matrix) != TOP_LEVEL_KEYS:
        errors.append(
            "matrix top-level fields differ: "
            f"missing={sorted(TOP_LEVEL_KEYS - set(matrix))}, "
            f"unknown={sorted(set(matrix) - TOP_LEVEL_KEYS)}"
        )
    if matrix.get("schema_version") != 2:
        errors.append("schema_version must equal 2")
    if matrix.get("milestone_id") != "TS1" or matrix.get("program_issue") != 502:
        errors.append("TS1 must remain the issue #502 parity program")
    if matrix.get("milestone_number") != 7 or matrix.get("milestone_url") != "https://github.com/hyperledger-identus/sdk-rust/milestone/7":
        errors.append("TS1 must remain bound to GitHub milestone 7")
    status = matrix.get("status")
    if matrix.get("definition_issue") != 594 or status not in {"baseline", "active", "complete"}:
        errors.append("definition_issue must remain 594 and status must be baseline, active, or complete")

    source = matrix.get("source")
    inventory_source = inventory.get("source")
    if not isinstance(source, dict) or not isinstance(inventory_source, dict):
        errors.append("source tables must exist in matrix and inventory")
        source = {}
        inventory_source = {}
    if source.get("inventory") != str(INVENTORY_PATH):
        errors.append("source.inventory must name the canonical capability inventory")
    revision = source.get("revision")
    if not isinstance(revision, str) or not SHA_PATTERN.fullmatch(revision):
        errors.append("source.revision must be a full lowercase Git SHA")
    for field in ("repository", "revision", "package", "version"):
        if source.get(field) != inventory_source.get(field):
            errors.append(f"source.{field} must match the capability inventory")

    if matrix.get("delivery") != DELIVERY:
        errors.append("delivery must preserve the local-first capability-train contract")
    vocabulary = matrix.get("vocabulary")
    expected_vocabulary = {
        "relations": sorted(RELATIONS),
        "states": sorted(STATES),
        "dimensions": sorted(DIMENSIONS),
        "targets": sorted(TARGETS),
        "target_states": sorted(STATES),
        "legacy_public_names": sorted(LEGACY_PUBLIC_NAMES),
        "donor_test_dispositions": sorted(DONOR_TEST_DISPOSITIONS),
        "rust_evidence_dispositions": sorted(RUST_EVIDENCE_DISPOSITIONS),
    }
    if not isinstance(vocabulary, dict):
        errors.append("vocabulary must be a table")
    else:
        for field, expected in expected_vocabulary.items():
            actual = vocabulary.get(field)
            if not isinstance(actual, list) or sorted(actual) != expected:
                errors.append(f"vocabulary.{field} must contain the closed {field} set")

    if matrix.get("ci") != CI:
        errors.append("ci must preserve the exact Rust-core-first isolated target contract")

    target_rows = matrix.get("target")
    if not isinstance(target_rows, list) or not all(isinstance(item, dict) for item in target_rows):
        errors.append("target must be an array of tables")
        target_rows = []
    target_ids = [str(item.get("id")) for item in target_rows]
    target_duplicates = sorted(identifier for identifier, count in Counter(target_ids).items() if count > 1)
    if target_duplicates:
        errors.append(f"duplicate target IDs: {target_duplicates}")
    if set(target_ids) != set(TARGETS) or len(target_rows) != len(TARGETS):
        errors.append(f"target IDs must be exactly {list(TARGETS)}")
    elif target_ids != list(TARGETS):
        errors.append(f"target records must preserve order {list(TARGETS)}")
    target_by_id = {str(item.get("id")): item for item in target_rows}
    for target_id in TARGETS:
        row = target_by_id.get(target_id)
        if row is None:
            continue
        prefix = f"target {target_id}"
        if set(row) != TARGET_KEYS:
            errors.append(
                f"{prefix} fields differ: missing={sorted(TARGET_KEYS - set(row))}, "
                f"unknown={sorted(set(row) - TARGET_KEYS)}"
            )
        expected_state = "blocked" if target_id == "react-native" else "target"
        if row.get("state") != expected_state:
            errors.append(f"{prefix}.state must equal {expected_state!r} at the baseline")
        expected_job = f"target-{target_id}"
        if row.get("ci_job") != expected_job:
            errors.append(f"{prefix}.ci_job must equal {expected_job!r}")
        for field in ("binding", "artifact", "module_format", "loader", "consumer_fixture", "support_tier"):
            text(row.get(field), f"{prefix}.{field}", errors)
        for field in ("runtime_matrix", "host_ports", "success_evidence", "limitations", "non_claims"):
            strings(row.get(field), f"{prefix}.{field}", errors)
        binding = str(row.get("binding", "")).lower()
        if target_id == "react-native":
            if "wasm" in binding or "native" not in binding or "turbomodule" not in binding:
                errors.append("target react-native must use the ADR 0163 native TurboModule route, never WASM")
            if "ADR-0163" not in row.get("limitations", []):
                errors.append("target react-native must retain the ADR-0163 limitation")
        elif "wasm" not in binding:
            errors.append(f"{prefix}.binding must retain its WASM route")

    criteria = matrix.get("success_criterion")
    if not isinstance(criteria, list) or not all(isinstance(item, dict) for item in criteria):
        errors.append("success_criterion must be an array of tables")
        criteria = []
    criterion_ids = [str(item.get("id")) for item in criteria]
    criterion_duplicates = sorted(identifier for identifier, count in Counter(criterion_ids).items() if count > 1)
    if criterion_duplicates:
        errors.append(f"duplicate success criterion IDs: {criterion_duplicates}")
    if set(criterion_ids) != set(SUCCESS_CRITERIA) or len(criteria) != len(SUCCESS_CRITERIA):
        errors.append(f"success criterion IDs must be exactly {list(SUCCESS_CRITERIA)}")
    elif criterion_ids != list(SUCCESS_CRITERIA):
        errors.append(f"success criterion records must preserve order {list(SUCCESS_CRITERIA)}")
    for index, criterion in enumerate(criteria, start=1):
        identifier = str(criterion.get("id"))
        prefix = f"success_criterion[{index}] {identifier}"
        if set(criterion) != SUCCESS_CRITERION_KEYS:
            errors.append(
                f"{prefix} fields differ: missing={sorted(SUCCESS_CRITERION_KEYS - set(criterion))}, "
                f"unknown={sorted(set(criterion) - SUCCESS_CRITERION_KEYS)}"
            )
        for field in ("title", "pass_condition", "applicability", "not_applicable_policy"):
            text(criterion.get(field), f"{prefix}.{field}", errors)
        for field in ("evidence", "non_claims"):
            strings(criterion.get(field), f"{prefix}.{field}", errors)
        expected_contract = SUCCESS_CRITERIA_CONTRACT.get(identifier)
        actual_contract = (
            criterion.get("title"),
            criterion.get("pass_condition"),
            tuple(criterion.get("evidence", [])) if isinstance(criterion.get("evidence"), list) else (),
            criterion.get("applicability"),
            criterion.get("not_applicable_policy"),
            tuple(criterion.get("non_claims", [])) if isinstance(criterion.get("non_claims"), list) else (),
        )
        if expected_contract is not None and actual_contract != expected_contract:
            errors.append(f"{prefix} must preserve the exact accepted success semantics")

    inventory_rows = inventory.get("capability")
    rows = matrix.get("capability")
    if not isinstance(inventory_rows, list) or not all(isinstance(item, dict) for item in inventory_rows):
        errors.append("inventory.capability must be an array of tables")
        inventory_rows = []
    if not isinstance(rows, list) or not all(isinstance(item, dict) for item in rows):
        errors.append("matrix capability must be an array of tables")
        rows = []

    inventory_by_id = {str(item.get("id")): item for item in inventory_rows}
    identifiers = [str(item.get("id")) for item in rows]
    duplicates = sorted(identifier for identifier, count in Counter(identifiers).items() if count > 1)
    if duplicates:
        errors.append(f"duplicate capability IDs: {duplicates}")
    if set(identifiers) != set(inventory_by_id):
        errors.append(
            "matrix capability coverage differs: "
            f"missing={sorted(set(inventory_by_id) - set(identifiers))}, "
            f"extra={sorted(set(identifiers) - set(inventory_by_id))}"
        )

    counts: Counter[str] = Counter()
    relation_counts: Counter[str] = Counter()
    state_members = {state: set() for state in STATES}
    relation_members = {relation: set() for relation in RELATIONS}
    for index, row in enumerate(rows, start=1):
        identifier = str(row.get("id"))
        prefix = f"capability[{index}] {identifier}"
        if set(row) != CAPABILITY_KEYS:
            errors.append(
                f"{prefix} fields differ: missing={sorted(CAPABILITY_KEYS - set(row))}, "
                f"unknown={sorted(set(row) - CAPABILITY_KEYS)}"
            )
        relation = row.get("relation")
        state = row.get("state")
        if relation not in RELATIONS:
            errors.append(f"{prefix}: unknown relation {relation!r}")
        if state not in STATES:
            errors.append(f"{prefix}: unknown state {state!r}")
        counts[str(state)] += 1
        relation_counts[str(relation)] += 1
        if state in STATES:
            state_members[str(state)].add(identifier)
        if relation in RELATIONS:
            relation_members[str(relation)].add(identifier)
        text(row.get("rationale"), f"{prefix}.rationale", errors)
        required = set(strings(row.get("required_dimensions"), f"{prefix}.required_dimensions", errors))
        if required - DIMENSIONS:
            errors.append(f"{prefix}: unknown dimensions {sorted(required - DIMENSIONS)}")
        missing = set(strings(row.get("missing_dimensions"), f"{prefix}.missing_dimensions", errors, allow_empty=True))
        if missing - DIMENSIONS:
            errors.append(f"{prefix}: unknown missing dimensions {sorted(missing - DIMENSIONS)}")
        if missing - required:
            errors.append(f"{prefix}: missing_dimensions must be a subset of required_dimensions")
        deviations = strings(row.get("known_deviations"), f"{prefix}.known_deviations", errors)
        strings(row.get("evidence"), f"{prefix}.evidence", errors)
        strings(row.get("non_claims"), f"{prefix}.non_claims", errors)
        for field in ("next_action", "consumer", "rollback", "rust_surface", "target_surface", "missing_proof"):
            text(row.get(field), f"{prefix}.{field}", errors)
        missing_proof = str(row.get("missing_proof", "")).strip()
        if not isinstance(row.get("owner_issue"), int) or row["owner_issue"] <= 0:
            errors.append(f"{prefix}.owner_issue must be a positive issue number")

        applicable_targets = strings(row.get("applicable_targets"), f"{prefix}.applicable_targets", errors)
        if len(set(applicable_targets)) != len(applicable_targets):
            errors.append(f"{prefix}.applicable_targets must not contain duplicates")
        unknown_targets = set(applicable_targets) - set(TARGETS)
        if unknown_targets:
            errors.append(f"{prefix}: unknown applicable targets {sorted(unknown_targets)}")
        target_states = row.get("target_states")
        if not isinstance(target_states, dict):
            errors.append(f"{prefix}.target_states must be a target-state table")
            target_states = {}
        if set(target_states) != set(TARGETS):
            errors.append(
                f"{prefix}.target_states keys differ: missing={sorted(set(TARGETS) - set(target_states))}, "
                f"unknown={sorted(set(target_states) - set(TARGETS))}"
            )
        for target_id in TARGETS:
            target_state = target_states.get(target_id)
            if target_state not in STATES:
                errors.append(f"{prefix}.target_states.{target_id} has unknown state {target_state!r}")
                continue
            if target_id in applicable_targets and target_state == "not-applicable":
                errors.append(f"{prefix}: applicable target {target_id} cannot be not-applicable")
            if target_id not in applicable_targets and target_state != "not-applicable":
                errors.append(f"{prefix}: non-applicable target {target_id} must be not-applicable")

        inventory_row = inventory_by_id.get(identifier, {})
        disposition = inventory_row.get("disposition") if isinstance(inventory_row, dict) else None
        if relation == "platform-owned" and disposition != "retain-platform":
            errors.append(f"{prefix}: platform-owned requires retain-platform inventory ownership")
        if state == "not-applicable" and relation != "platform-owned":
            errors.append(f"{prefix}: not-applicable requires platform-owned relation")
        if relation == "platform-owned" and state != "not-applicable":
            errors.append(f"{prefix}: platform-owned must be explicitly not-applicable to semantic parity")
        if relation == "intentionally-different" and not deviations:
            errors.append(f"{prefix}: intentional difference requires an explicit deviation")
        if relation in {"deferred", "retired"} and state not in {"blocked", "excluded"}:
            errors.append(f"{prefix}: {relation} must remain blocked or excluded")
        if state == "delivered":
            missing_dimensions = DIMENSIONS - required
            if missing_dimensions:
                errors.append(f"{prefix}: delivered claim lacks dimensions {sorted(missing_dimensions)}")
            if missing:
                errors.append(f"{prefix}: delivered claim retains missing dimensions {sorted(missing)}")
            undelivered_targets = sorted(
                target_id for target_id in applicable_targets if target_states.get(target_id) != "delivered"
            )
            if undelivered_targets:
                errors.append(f"{prefix}: delivered claim has undelivered applicable targets {undelivered_targets}")
            if "not activated" in str(row.get("next_action", "")).lower():
                errors.append(f"{prefix}: delivered claim contradicts its next action")
            if missing_proof.lower() != "none":
                errors.append(f"{prefix}: delivered claim must set missing_proof to 'none'")
        else:
            if state == "excluded":
                non_excluded_targets = sorted(
                    target_id for target_id in applicable_targets if target_states.get(target_id) != "excluded"
                )
                if non_excluded_targets:
                    errors.append(f"{prefix}: excluded claim has non-excluded applicable targets {non_excluded_targets}")
            if not missing:
                errors.append(f"{prefix}: non-delivered capability must identify missing_dimensions")
            elif missing_proof.lower() == "none":
                errors.append(f"{prefix}: non-delivered capability cannot set missing_proof to 'none'")

    if not rows or sum(counts[state] for state in STATES) != len(rows):
        errors.append("every capability must have exactly one recognized conformance state")
    if status == "baseline" and (counts["delivered"] or counts["excluded"]):
        errors.append("baseline must not claim delivered or excluded SDK-TS routes")
    if status == "complete":
        errors.append("schema_version 2 cannot claim complete before an explicit success-evidence receipt schema exists")

    expected_status_line = (
        f"**Status:** matrix status = `{status}`; delivered capabilities = "
        f"{counts['delivered']}; no runtime support is claimed"
    )
    if expected_status_line not in report.splitlines():
        errors.append("report status line must match matrix status and delivered count")

    report_states = report_group_rows(report, STATES, "state", errors)
    report_relations = report_group_rows(report, RELATIONS, "relation", errors)
    for state in STATES:
        if report_states.get(state) != (counts[state], state_members[state]):
            errors.append(f"report state row {state} must match the machine matrix")
    for relation in RELATIONS:
        if report_relations.get(relation) != (relation_counts[relation], relation_members[relation]):
            errors.append(f"report relation row {relation} must match the machine matrix")

    target_report_rows = table_rows_between_markers(
        report,
        "target-contract",
        [
            "Target",
            "State",
            "Runtime matrix",
            "Binding",
            "Artifact",
            "Module format",
            "Loader",
            "Host ports",
            "Consumer fixture",
            "Support tier",
            "CI job",
            "Success evidence",
            "Limits and non-claims",
        ],
        errors,
    )
    if len(target_report_rows) != len(target_rows):
        errors.append("report target-contract row count must match the machine target records")
    for index, target_row in enumerate(target_rows):
        if index >= len(target_report_rows):
            break
        actual = [unquote_code(cell) for cell in target_report_rows[index]]
        if actual != expected_target_report_row(target_row):
            errors.append(f"report target-contract row {target_row.get('id')} must exactly match the machine target")

    success_report_rows = table_rows_between_markers(
        report,
        "success-criteria",
        ["ID", "Title", "Pass condition", "Evidence", "Applicability", "Not-applicable policy", "Non-claims"],
        errors,
    )
    if len(success_report_rows) != len(criteria):
        errors.append("report success-criteria row count must match the machine success criteria")
    for index, criterion in enumerate(criteria):
        if index >= len(success_report_rows):
            break
        actual = [unquote_code(cell) for cell in success_report_rows[index]]
        if actual != expected_success_report_row(criterion):
            errors.append(f"report success-criteria row {criterion.get('id')} must exactly match the machine criterion")

    feature_rows = table_rows_between_markers(
        report,
        "feature-comparison",
        [
            "Capability",
            "Rust today",
            "SDK-TS today",
            "Rust owner",
            "TypeScript owner",
            "Target",
            "B/N/RW/RN",
            "Deviation / missing proof",
        ],
        errors,
    )
    report_features: dict[str, list[str]] = {}
    for cells in feature_rows:
        if len(cells) != 8:
            errors.append(f"report feature-comparison row must have 8 cells: {cells}")
            continue
        identifier = unquote_code(cells[0])
        if identifier in report_features:
            errors.append(f"report feature-comparison duplicates {identifier}")
        report_features[identifier] = cells
    if set(report_features) != set(identifiers):
        errors.append(
            "report feature-comparison coverage differs: "
            f"missing={sorted(set(identifiers) - set(report_features))}, "
            f"extra={sorted(set(report_features) - set(identifiers))}"
        )
    rows_by_id = {str(row.get("id")): row for row in rows}
    for identifier in identifiers:
        cells = report_features.get(identifier)
        row = rows_by_id.get(identifier)
        inventory_row = inventory_by_id.get(identifier)
        if cells is None or row is None or not isinstance(inventory_row, dict):
            continue
        expected = [
            identifier,
            str(row.get("rust_surface")),
            str(inventory_row.get("outcome")),
            str(inventory_row.get("target_owner")),
            str(inventory_row.get("typescript_owner")),
            str(row.get("target_surface")),
            expected_target_summary(row.get("target_states") if isinstance(row.get("target_states"), dict) else {}),
            expected_gap_summary(row),
        ]
        actual = [unquote_code(cells[0]), *cells[1:]]
        if actual != expected:
            errors.append(f"report feature-comparison row {identifier} must match the machine matrix and donor inventory")

    if errors:
        for error in errors:
            print(f"sdk-ts-parity-conformance: {error}", file=sys.stderr)
        return 1
    print(
        "sdk-ts-parity-conformance: "
        f"{len(rows)} capabilities passed "
        f"(partial={counts['partial']}, blocked={counts['blocked']}, "
        f"not-applicable={counts['not-applicable']})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
