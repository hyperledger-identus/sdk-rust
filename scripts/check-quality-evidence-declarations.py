#!/usr/bin/env python3
"""Validate and render the closed quality-evidence declaration registry."""

from __future__ import annotations

import argparse
import datetime as dt
from pathlib import Path
import re
import stat
import sys
import tomllib
from typing import Any


REGISTRY = Path("docs/architecture/quality-evidence-declarations.toml")
PLAN = Path("docs/architecture/quality-evidence-plan.md")
VECTOR_CATALOG = Path("docs/conformance/cross-language-vector-catalog.toml")

TOP_KEYS = {
    "schema_version",
    "registry_version",
    "status",
    "owner_issue",
    "evaluated_at",
    "allowed_classes",
    "allowed_lanes",
    "allowed_cadences",
    "declarations",
}
DECLARATION_KEYS = {
    "id",
    "capability",
    "slice",
    "owner",
    "owner_issue",
    "risk",
    "targets",
    "vector_ids",
    "limitations",
    "active",
    "supersedes",
    "replaced_by",
    "obligations",
}
OBLIGATION_KEYS = {
    "class",
    "disposition",
    "risk",
    "selectors",
    "command",
    "lane",
    "cadence",
    "targets",
    "freshness",
    "freshness_days",
    "budget",
    "receipt_kind",
    "receipt",
    "evidence_revision",
    "recorded_at",
    "artifact",
    "owner",
    "owner_issue",
    "debt_status",
    "debt_issue",
    "rationale",
    "details",
}

CLASSES = ["property", "fuzz", "benchmark", "differential"]
LANES = ["focused", "fast", "slow", "release", "none"]
CADENCES = [
    "on-change",
    "per-pull-request",
    "weekly-or-manual",
    "per-candidate",
    "not-applicable",
]
ROUTES = {
    ("focused", "on-change"),
    ("fast", "per-pull-request"),
    ("slow", "weekly-or-manual"),
    ("release", "per-candidate"),
}
DETAIL_KEYS = {
    "property": {"invariant", "domain", "cases"},
    "fuzz": {"harness", "corpus", "seed", "bounds", "sanitizer"},
    "benchmark": {
        "operation",
        "fixture",
        "environment",
        "statistic",
        "samples",
        "threshold",
    },
    "differential": {
        "oracle",
        "oracle-revision",
        "normalization",
        "deviations",
    },
}

ID_RE = re.compile(r"^[a-z][a-z0-9]*(?:[.-][a-z0-9]+)*$")
TARGET_RE = re.compile(r"^[a-z][a-z0-9-]*:[A-Za-z0-9][A-Za-z0-9._-]*$")
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
SEMVER_RE = re.compile(r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$")
RUN_RE = re.compile(
    r"^https://github\.com/hyperledger-identus/sdk-rust/actions/runs/[1-9][0-9]*$"
)


def fail(errors: list[str], context: str, message: str) -> None:
    errors.append(f"{context}: {message}")


def exact_keys(value: Any, expected: set[str], context: str, errors: list[str]) -> bool:
    if not isinstance(value, dict):
        fail(errors, context, "must be a table")
        return False
    actual = set(value)
    missing = sorted(expected - actual)
    unknown = sorted(actual - expected)
    if missing:
        fail(errors, context, f"missing keys: {', '.join(missing)}")
    if unknown:
        fail(errors, context, f"unknown keys: {', '.join(unknown)}")
    return not missing and not unknown


def bounded_text(
    value: Any,
    context: str,
    errors: list[str],
    *,
    minimum: int = 1,
    maximum: int = 1024,
) -> str:
    if not isinstance(value, str):
        fail(errors, context, "must be a string")
        return ""
    if len(value) < minimum or len(value) > maximum:
        fail(errors, context, f"must contain {minimum}..{maximum} characters")
    if any(ord(character) < 32 for character in value):
        fail(errors, context, "must not contain control characters or newlines")
    return value


def positive_int(value: Any, context: str, errors: list[str], *, allow_zero: bool = False) -> int:
    minimum = 0 if allow_zero else 1
    if isinstance(value, bool) or not isinstance(value, int) or value < minimum:
        fail(errors, context, f"must be an integer >= {minimum}")
        return 0
    return value


def string_list(
    value: Any,
    context: str,
    errors: list[str],
    *,
    allow_empty: bool = False,
    maximum: int = 128,
) -> list[str]:
    if not isinstance(value, list):
        fail(errors, context, "must be an array")
        return []
    if not allow_empty and not value:
        fail(errors, context, "must not be empty")
    if len(value) > maximum:
        fail(errors, context, f"must contain at most {maximum} entries")
    result: list[str] = []
    for index, item in enumerate(value):
        text = bounded_text(item, f"{context}[{index}]", errors, maximum=512)
        if text:
            result.append(text)
    if len(set(result)) != len(result):
        fail(errors, context, "must not contain duplicates")
    return result


def safe_file(root: Path, raw: str, context: str, errors: list[str], *, max_bytes: int) -> Path | None:
    path = Path(raw)
    if path.is_absolute() or ".." in path.parts or raw in {"", "."}:
        fail(errors, context, "must be a safe repository-relative file path")
        return None
    candidate = root / path
    try:
        metadata = candidate.lstat()
    except FileNotFoundError:
        fail(errors, context, f"file does not exist: {raw}")
        return None
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(errors, context, f"must be a regular non-symlink file: {raw}")
        return None
    if metadata.st_size > max_bytes:
        fail(errors, context, f"file exceeds {max_bytes} bytes: {raw}")
        return None
    try:
        candidate.resolve().relative_to(root.resolve())
    except ValueError:
        fail(errors, context, f"file escapes repository root: {raw}")
        return None
    return candidate


def load_toml(root: Path, relative: Path, context: str, errors: list[str]) -> dict[str, Any]:
    path = safe_file(root, relative.as_posix(), context, errors, max_bytes=1_000_000)
    if path is None:
        return {}
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
        fail(errors, context, f"cannot parse TOML: {error}")
        return {}


def parse_date(value: Any, context: str, errors: list[str]) -> dt.date | None:
    text = bounded_text(value, context, errors, maximum=10)
    try:
        parsed = dt.date.fromisoformat(text)
    except ValueError:
        fail(errors, context, "must be an ISO 8601 calendar date")
        return None
    if parsed.isoformat() != text:
        fail(errors, context, "must be a canonical ISO 8601 calendar date")
        return None
    return parsed


def validate_selector(root: Path, selector: str, context: str, errors: list[str]) -> None:
    if "#" not in selector:
        fail(errors, context, "must use repository/path#needle syntax")
        return
    raw_path, needle = selector.rsplit("#", 1)
    if not needle or len(needle) > 160 or "\n" in needle:
        fail(errors, context, "must contain a bounded non-empty selector needle")
        return
    path = safe_file(root, raw_path, context, errors, max_bytes=1_000_000)
    if path is None:
        return
    try:
        content = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(errors, context, f"cannot read UTF-8 selector source: {error}")
        return
    if needle not in content:
        fail(errors, context, f"selector needle not found in {raw_path}: {needle}")


def validate_details(
    evidence_class: str,
    details: list[str],
    context: str,
    errors: list[str],
) -> None:
    parsed: dict[str, str] = {}
    for index, detail in enumerate(details):
        if "=" not in detail:
            fail(errors, f"{context}[{index}]", "must use key=value syntax")
            continue
        key, value = detail.split("=", 1)
        if key in parsed:
            fail(errors, context, f"duplicate detail key: {key}")
        if not value.strip():
            fail(errors, f"{context}.{key}", "must not be empty")
        parsed[key] = value
    expected = DETAIL_KEYS.get(evidence_class, set())
    if set(parsed) != expected:
        missing = sorted(expected - set(parsed))
        unknown = sorted(set(parsed) - expected)
        if missing:
            fail(errors, context, f"missing detail keys: {', '.join(missing)}")
        if unknown:
            fail(errors, context, f"unknown detail keys: {', '.join(unknown)}")


def validate_obligation(
    root: Path,
    obligation: Any,
    expected_class: str,
    evaluated_at: dt.date | None,
    declaration_owner_issue: int,
    context: str,
    errors: list[str],
) -> None:
    if not exact_keys(obligation, OBLIGATION_KEYS, context, errors):
        return
    evidence_class = bounded_text(obligation["class"], f"{context}.class", errors, maximum=32)
    if evidence_class != expected_class:
        fail(errors, f"{context}.class", f"must be {expected_class}")
    disposition = bounded_text(
        obligation["disposition"], f"{context}.disposition", errors, maximum=32
    )
    if disposition not in {"satisfied", "required", "not-applicable"}:
        fail(errors, f"{context}.disposition", "must be satisfied, required, or not-applicable")
    bounded_text(obligation["risk"], f"{context}.risk", errors, minimum=24, maximum=640)
    selectors = string_list(
        obligation["selectors"], f"{context}.selectors", errors, allow_empty=True, maximum=16
    )
    command = bounded_text(
        obligation["command"], f"{context}.command", errors, minimum=0, maximum=512
    )
    lane = bounded_text(obligation["lane"], f"{context}.lane", errors, maximum=16)
    cadence = bounded_text(obligation["cadence"], f"{context}.cadence", errors, maximum=32)
    targets = string_list(
        obligation["targets"], f"{context}.targets", errors, allow_empty=True, maximum=16
    )
    for index, target in enumerate(targets):
        if not TARGET_RE.fullmatch(target):
            fail(errors, f"{context}.targets[{index}]", "must use kind:value syntax")
    freshness = bounded_text(
        obligation["freshness"], f"{context}.freshness", errors, maximum=32
    )
    freshness_days = positive_int(
        obligation["freshness_days"], f"{context}.freshness_days", errors, allow_zero=True
    )
    budget = bounded_text(
        obligation["budget"], f"{context}.budget", errors, minimum=0, maximum=640
    )
    receipt_kind = bounded_text(
        obligation["receipt_kind"], f"{context}.receipt_kind", errors, maximum=32
    )
    receipt = bounded_text(
        obligation["receipt"], f"{context}.receipt", errors, minimum=0, maximum=512
    )
    evidence_revision = bounded_text(
        obligation["evidence_revision"],
        f"{context}.evidence_revision",
        errors,
        minimum=0,
        maximum=40,
    )
    recorded_text = bounded_text(
        obligation["recorded_at"], f"{context}.recorded_at", errors, minimum=0, maximum=10
    )
    bounded_text(obligation["artifact"], f"{context}.artifact", errors, minimum=0, maximum=512)
    owner = bounded_text(obligation["owner"], f"{context}.owner", errors, maximum=160)
    owner_issue = positive_int(obligation["owner_issue"], f"{context}.owner_issue", errors)
    debt_status = bounded_text(
        obligation["debt_status"], f"{context}.debt_status", errors, maximum=16
    )
    debt_issue = positive_int(
        obligation["debt_issue"], f"{context}.debt_issue", errors, allow_zero=True
    )
    rationale = bounded_text(
        obligation["rationale"], f"{context}.rationale", errors, minimum=0, maximum=720
    )
    details = string_list(
        obligation["details"], f"{context}.details", errors, allow_empty=True, maximum=12
    )

    if not owner.strip() or owner_issue <= 0:
        fail(errors, context, "must have an accountable owner and positive owner issue")
    if owner_issue != declaration_owner_issue:
        fail(errors, f"{context}.owner_issue", "must match the declaration owner issue")
    if lane not in LANES or cadence not in CADENCES:
        fail(errors, context, "uses a lane or cadence outside the closed registry vocabulary")

    if disposition == "not-applicable":
        if (lane, cadence) != ("none", "not-applicable"):
            fail(errors, context, "not-applicable evidence must use none/not-applicable routing")
        empty_fields = {
            "selectors": selectors,
            "command": command,
            "targets": targets,
            "budget": budget,
            "receipt": receipt,
            "evidence_revision": evidence_revision,
            "recorded_at": recorded_text,
            "artifact": obligation["artifact"],
            "details": details,
        }
        for field, value in empty_fields.items():
            if value:
                fail(errors, f"{context}.{field}", "must be empty for not-applicable evidence")
        if freshness != "not-applicable" or freshness_days != 0 or receipt_kind != "none":
            fail(errors, context, "not-applicable evidence must not claim freshness or a receipt")
        if len(rationale.strip()) < 48:
            fail(errors, f"{context}.rationale", "must explain the bounded omission in at least 48 characters")
        if debt_status != "none" or debt_issue != 0:
            fail(errors, context, "not-applicable evidence must not conceal delivery debt")
        return

    if (lane, cadence) not in ROUTES:
        fail(errors, context, "must use a deterministic focused, fast, slow, or release route")
    if not selectors or not command.strip() or not targets or not budget.strip():
        fail(errors, context, "executable evidence requires selectors, command, targets, and budget")
    for index, selector in enumerate(selectors):
        validate_selector(root, selector, f"{context}.selectors[{index}]", errors)
    validate_details(evidence_class, details, f"{context}.details", errors)
    if rationale:
        fail(errors, f"{context}.rationale", "must be empty unless evidence is not applicable")

    if disposition == "required":
        if receipt_kind != "none" or receipt or evidence_revision or recorded_text:
            fail(errors, context, "required evidence must not claim a receipt or evidence revision")
        if freshness != "pending" or freshness_days != 0:
            fail(errors, context, "required evidence must declare pending freshness")
        if debt_status != "open" or debt_issue <= 0:
            fail(errors, context, "required evidence must have open debt with a positive debt issue")
        return

    if debt_status != "none" or debt_issue != 0:
        fail(errors, context, "satisfied evidence must not retain unowned or open debt")
    if not SHA_RE.fullmatch(evidence_revision):
        fail(errors, f"{context}.evidence_revision", "must be a full lowercase 40-character commit SHA")
    recorded_at = parse_date(recorded_text, f"{context}.recorded_at", errors)
    if evaluated_at is not None and recorded_at is not None and recorded_at > evaluated_at:
        fail(errors, f"{context}.recorded_at", "must not be later than the registry evaluation date")
    if receipt_kind == "source":
        if freshness != "source-bound" or freshness_days != 0:
            fail(errors, context, "source receipts must use source-bound freshness with zero days")
        if not receipt.startswith("source:"):
            fail(errors, f"{context}.receipt", "must use source:repository/path syntax")
        else:
            safe_file(root, receipt[7:], f"{context}.receipt", errors, max_bytes=1_000_000)
    elif receipt_kind == "github-run":
        if not RUN_RE.fullmatch(receipt):
            fail(errors, f"{context}.receipt", "must be an immutable sdk-rust GitHub Actions run URL")
        if freshness == "max-age-days":
            if freshness_days <= 0:
                fail(errors, f"{context}.freshness_days", "must be positive for max-age-days")
            if evaluated_at is not None and recorded_at is not None:
                age = (evaluated_at - recorded_at).days
                if age < 0 or age > freshness_days:
                    fail(
                        errors,
                        f"{context}.recorded_at",
                        f"receipt is stale or future-dated at evaluation ({age} days; maximum {freshness_days})",
                    )
        elif freshness == "exact-head":
            if freshness_days != 0:
                fail(errors, f"{context}.freshness_days", "must be zero for exact-head evidence")
        else:
            fail(errors, f"{context}.freshness", "GitHub receipts require max-age-days or exact-head")
    else:
        fail(errors, f"{context}.receipt_kind", "satisfied evidence requires source or github-run")


def validate_registry(root: Path, errors: list[str]) -> dict[str, Any]:
    registry = load_toml(root, REGISTRY, "registry", errors)
    if not exact_keys(registry, TOP_KEYS, "registry", errors):
        return registry
    if registry["schema_version"] != 1:
        fail(errors, "registry.schema_version", "must be 1")
    version = bounded_text(registry["registry_version"], "registry.registry_version", errors, maximum=32)
    if not SEMVER_RE.fullmatch(version):
        fail(errors, "registry.registry_version", "must be canonical major.minor.patch SemVer")
    if registry["status"] != "active":
        fail(errors, "registry.status", "must be active")
    positive_int(registry["owner_issue"], "registry.owner_issue", errors)
    evaluated_at = parse_date(registry["evaluated_at"], "registry.evaluated_at", errors)
    if registry["allowed_classes"] != CLASSES:
        fail(errors, "registry.allowed_classes", "must equal the closed ordered evidence classes")
    if registry["allowed_lanes"] != LANES:
        fail(errors, "registry.allowed_lanes", "must equal the closed ordered lanes")
    if registry["allowed_cadences"] != CADENCES:
        fail(errors, "registry.allowed_cadences", "must equal the closed ordered cadences")

    catalog = load_toml(root, VECTOR_CATALOG, "vector catalog", errors)
    vector_ids = {
        vector.get("id")
        for vector in catalog.get("vectors", [])
        if isinstance(vector, dict) and isinstance(vector.get("id"), str)
    }
    declarations = registry["declarations"]
    if not isinstance(declarations, list) or not declarations:
        fail(errors, "registry.declarations", "must be a non-empty array of tables")
        return registry
    declaration_ids: set[str] = set()
    lifecycle: dict[str, list[str]] = {}
    for index, declaration in enumerate(declarations):
        context = f"declarations[{index}]"
        if not exact_keys(declaration, DECLARATION_KEYS, context, errors):
            continue
        declaration_id = bounded_text(declaration["id"], f"{context}.id", errors, maximum=96)
        if not ID_RE.fullmatch(declaration_id):
            fail(errors, f"{context}.id", "must be a stable lowercase dotted identifier")
        if declaration_id in declaration_ids:
            fail(errors, f"{context}.id", "must be unique")
        declaration_ids.add(declaration_id)
        capability = bounded_text(
            declaration["capability"], f"{context}.capability", errors, maximum=96
        )
        if not ID_RE.fullmatch(capability):
            fail(errors, f"{context}.capability", "must be a lowercase dotted identifier")
        bounded_text(declaration["slice"], f"{context}.slice", errors, maximum=160)
        bounded_text(declaration["owner"], f"{context}.owner", errors, maximum=160)
        owner_issue = positive_int(declaration["owner_issue"], f"{context}.owner_issue", errors)
        bounded_text(declaration["risk"], f"{context}.risk", errors, minimum=24, maximum=640)
        targets = string_list(declaration["targets"], f"{context}.targets", errors, maximum=24)
        for target_index, target in enumerate(targets):
            if not TARGET_RE.fullmatch(target):
                fail(errors, f"{context}.targets[{target_index}]", "must use kind:value syntax")
        declared_vectors = string_list(
            declaration["vector_ids"], f"{context}.vector_ids", errors, allow_empty=True
        )
        for vector_id in declared_vectors:
            if vector_id not in vector_ids:
                fail(errors, f"{context}.vector_ids", f"unknown local vector id: {vector_id}")
        bounded_text(
            declaration["limitations"], f"{context}.limitations", errors, minimum=24, maximum=720
        )
        if not isinstance(declaration["active"], bool):
            fail(errors, f"{context}.active", "must be a boolean")
        supersedes = string_list(
            declaration["supersedes"], f"{context}.supersedes", errors, allow_empty=True
        )
        replaced_by = bounded_text(
            declaration["replaced_by"], f"{context}.replaced_by", errors, minimum=0, maximum=96
        )
        lifecycle[declaration_id] = supersedes + ([replaced_by] if replaced_by else [])
        obligations = declaration["obligations"]
        if not isinstance(obligations, list) or len(obligations) != len(CLASSES):
            fail(errors, f"{context}.obligations", "must contain exactly four ordered evidence classes")
            continue
        for obligation_index, expected_class in enumerate(CLASSES):
            validate_obligation(
                root,
                obligations[obligation_index],
                expected_class,
                evaluated_at,
                owner_issue,
                f"{context}.obligations[{obligation_index}]",
                errors,
            )

    for declaration_id, references in lifecycle.items():
        for reference in references:
            if reference == declaration_id:
                fail(errors, declaration_id, "lifecycle references must not point to self")
            elif reference not in declaration_ids:
                fail(errors, declaration_id, f"unknown lifecycle declaration: {reference}")
    visited: set[str] = set()

    def visit(current: str, visiting: set[str]) -> None:
        if current in visiting:
            fail(errors, current, "lifecycle references must be acyclic")
            return
        if current in visited:
            return
        visiting.add(current)
        for reference in lifecycle.get(current, []):
            if reference in lifecycle:
                visit(reference, visiting)
        visiting.remove(current)
        visited.add(current)

    for declaration_id in lifecycle:
        visit(declaration_id, set())
    return registry


def render_plan(registry: dict[str, Any], capability: str | None = None) -> str:
    declarations = registry.get("declarations", [])
    if capability:
        declarations = [item for item in declarations if item.get("capability") == capability]
    lines = [
        "# Quality evidence plan",
        "",
        "Generated deterministically from `docs/architecture/quality-evidence-declarations.toml`.",
        "Commands are declared evidence routes; rendering this plan never executes them.",
        "",
        f"- Registry version: `{registry.get('registry_version', '')}`",
        f"- Evidence evaluated: `{registry.get('evaluated_at', '')}`",
        f"- Declaration count: `{len(declarations)}`",
        "",
    ]
    for declaration in declarations:
        lines.extend(
            [
                f"## `{declaration['id']}`",
                "",
                f"- Capability: `{declaration['capability']}`",
                f"- Slice: {declaration['slice']}",
                f"- Owner: {declaration['owner']} (issue #{declaration['owner_issue']})",
                f"- Risk: {declaration['risk']}",
                f"- Targets: {', '.join(f'`{item}`' for item in declaration['targets'])}",
                f"- Vector IDs: `{len(declaration['vector_ids'])}` local catalog entries",
                f"- Limitations: {declaration['limitations']}",
                "",
                "| Class | Disposition | Lane | Cadence | Freshness | Owner issue |",
                "| --- | --- | --- | --- | --- | ---: |",
            ]
        )
        for obligation in declaration["obligations"]:
            freshness = obligation["freshness"]
            if obligation["freshness_days"]:
                freshness += f" ({obligation['freshness_days']} days)"
            lines.append(
                f"| `{obligation['class']}` | `{obligation['disposition']}` | "
                f"`{obligation['lane']}` | `{obligation['cadence']}` | "
                f"`{freshness}` | #{obligation['owner_issue']} |"
            )
        lines.append("")
        for obligation in declaration["obligations"]:
            lines.extend([f"### `{obligation['class']}`", ""])
            if obligation["disposition"] == "not-applicable":
                lines.append(f"- Rationale: {obligation['rationale']}")
            else:
                lines.extend(
                    [
                        f"- Command: `{obligation['command']}`",
                        f"- Budget: {obligation['budget']}",
                        f"- Receipt: `{obligation['receipt'] or 'pending'}`",
                        f"- Evidence revision: `{obligation['evidence_revision'] or 'pending'}`",
                    ]
                )
                if obligation["debt_status"] == "open":
                    lines.append(f"- Delivery debt: #{obligation['debt_issue']}")
            lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--render", action="store_true", help="print the complete deterministic plan")
    mode.add_argument(
        "--plan",
        nargs="?",
        const="",
        metavar="CAPABILITY",
        help="print a non-executing plan, optionally filtered by capability",
    )
    args = parser.parse_args()
    root = Path(args.root).resolve()
    errors: list[str] = []
    registry = validate_registry(root, errors)
    if args.plan not in {None, ""}:
        known = {item.get("capability") for item in registry.get("declarations", [])}
        if args.plan not in known:
            fail(errors, "plan", f"unknown capability: {args.plan}")
    if errors:
        for error in errors:
            print(f"quality-evidence: {error}", file=sys.stderr)
        return 1
    rendered = render_plan(registry, args.plan or None)
    if args.render or args.plan is not None:
        sys.stdout.write(rendered)
        return 0
    plan_path = safe_file(root, PLAN.as_posix(), "quality evidence plan", errors, max_bytes=1_000_000)
    if plan_path is None:
        for error in errors:
            print(f"quality-evidence: {error}", file=sys.stderr)
        return 1
    try:
        checked_in = plan_path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        print(f"quality-evidence: quality evidence plan: cannot read UTF-8: {error}", file=sys.stderr)
        return 1
    if checked_in != rendered:
        print(
            "quality-evidence: deterministic plan drift; regenerate with "
            "scripts/check-quality-evidence-declarations.py --render",
            file=sys.stderr,
        )
        return 1
    obligation_count = sum(len(item["obligations"]) for item in registry["declarations"])
    print(
        f"quality-evidence: {len(registry['declarations'])} declaration(s) and "
        f"{obligation_count} obligation(s) passed"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
