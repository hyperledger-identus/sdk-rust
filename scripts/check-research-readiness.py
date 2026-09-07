#!/usr/bin/env python3
"""Validate active OpenSpec research records without network access."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


ALLOWED_CLASSES = {
    "routine",
    "foundational",
    "protocol",
    "cryptography-security",
    "storage-ffi",
}
FULL_CLASSES = ALLOWED_CLASSES - {"routine"}
ALLOWED_DECISIONS = {
    "adopt",
    "conditional-adopt",
    "spike",
    "oracle",
    "retain-local",
    "not-adopt",
    "not-applicable",
}
REQUIRED_HEADINGS = (
    "Problem and existing implementation",
    "Normative sources",
    "Candidate decisions",
    "Compatibility and dependency evidence",
    "Security, privacy and maintenance evidence",
    "Rejected or deferred candidates",
    "Open questions and blockers",
    "Evidence commands",
)
FULL_EVIDENCE_TERMS = {
    "current implementation": ("current implementation",),
    "consumer evidence": ("consumer",),
    "primary source URL": ("https://",),
    "pinned source revision": ("revision",),
    "exact version and features": ("version", "feature"),
    "license and provenance": ("license", "provenance"),
    "MSRV": ("msrv",),
    "target evidence": ("target",),
    "direct and resolved dependency cone": (
        "direct",
        "resolved",
        "dependency cone",
    ),
    "unsafe and native-code evidence": ("unsafe", "native"),
    "supply-chain evidence": ("supply-chain",),
    "public and wire compatibility": ("public", "wire"),
    "facade boundary": ("facade",),
    "rollback": ("rollback",),
    "maintenance, release and security posture": (
        "maintenance",
        "release",
        "security",
    ),
    "protocol or draft currency": ("protocol", "draft"),
    "reconsideration trigger": ("reconsideration trigger",),
    "exact commands and unrun checks": ("command", "unrun"),
}


def field(text: str, name: str) -> str | None:
    match = re.search(rf"(?m)^{re.escape(name)}:\s*(\S.*)$", text)
    return match.group(1).strip() if match else None


def section_body(text: str, heading: str) -> str | None:
    match = re.search(
        rf"(?ms)^## {re.escape(heading)}\s*$\n(.*?)(?=^## |\Z)", text
    )
    return match.group(1).strip() if match else None


def validate_record(path: Path, require_ready: bool) -> list[str]:
    failures: list[str] = []
    context = f"active change {path.parent.name}"

    if not path.is_file():
        return [f"{context} is missing research.md"]

    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return [f"{context} research.md is not valid UTF-8"]

    research_class = field(text, "Research class")
    if research_class not in ALLOWED_CLASSES:
        failures.append(
            f"{context} has invalid Research class {research_class!r}; "
            f"expected one of {sorted(ALLOWED_CLASSES)}"
        )

    status = field(text, "Research status")
    if status not in {"draft", "ready"}:
        failures.append(f"{context} Research status must be draft or ready")
    elif require_ready and status != "ready":
        failures.append(f"{context} research is not ready")

    for date_field in ("Decision date", "Source retrieval date"):
        value = field(text, date_field)
        if value is None or re.fullmatch(r"\d{4}-\d{2}-\d{2}", value) is None:
            failures.append(f"{context} {date_field} must be YYYY-MM-DD")

    blockers = field(text, "Research blockers")
    if not blockers:
        failures.append(f"{context} must declare Research blockers")
    elif status == "ready" and blockers != "none":
        failures.append(
            f"{context} cannot be ready while Research blockers is {blockers!r}"
        )

    sections: dict[str, str] = {}
    for heading in REQUIRED_HEADINGS:
        body = section_body(text, heading)
        if body is None:
            failures.append(f"{context} is missing heading: {heading}")
        else:
            sections[heading] = body

    decisions = {
        decision
        for decision in ALLOWED_DECISIONS
        if re.search(rf"`{re.escape(decision)}`", text)
    }
    if not decisions:
        failures.append(f"{context} has no allowed candidate decision")

    if research_class in FULL_CLASSES:
        for heading, body in sections.items():
            normalized = body.strip().lower().rstrip(".")
            if normalized in {"", "...", "not applicable", "n/a"}:
                failures.append(
                    f"{context} full research section is empty: {heading}"
                )

        lowered = text.lower()
        for label, tokens in FULL_EVIDENCE_TERMS.items():
            if any(token not in lowered for token in tokens):
                failures.append(f"{context} full research is missing {label}")
        if decisions == {"not-applicable"}:
            failures.append(
                f"{context} full research cannot mark every candidate not-applicable"
            )

    return failures


def active_changes(root: Path, selected: str | None) -> list[Path]:
    changes_root = root / "openspec" / "changes"
    if selected:
        return [changes_root / selected]
    if not changes_root.is_dir():
        return []
    return sorted(
        path
        for path in changes_root.iterdir()
        if path.is_dir() and path.name != "archive"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--change")
    parser.add_argument("--require-ready", action="store_true")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    changes = active_changes(root, args.change)
    failures: list[str] = []

    if args.change and not changes[0].is_dir():
        failures.append(f"active change not found: {args.change}")
    else:
        for change in changes:
            failures.extend(
                validate_record(change / "research.md", args.require_ready)
            )

    if failures:
        for failure in failures:
            print(f"research-readiness: {failure}", file=sys.stderr)
        print(
            f"research-readiness: {len(failures)} failure(s)", file=sys.stderr
        )
        return 1

    qualifier = "ready" if args.require_ready else "structure"
    print(f"research-readiness: {qualifier} passed for {len(changes)} change(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
