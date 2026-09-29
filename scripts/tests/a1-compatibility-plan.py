#!/usr/bin/env python3
"""Mutation tests for the A1 compatibility-foundation plan checker."""

from __future__ import annotations

import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-a1-compatibility-plan.py"
PLAN = ROOT / "docs/roadmap/a1-shared-compatibility-foundation.toml"
ROADMAP = ROOT / "docs/roadmap/platform-core-adoption-milestones.toml"


def replace_once(text: str, before: str, after: str) -> str:
    count = text.count(before)
    if count != 1:
        raise AssertionError(f"mutation source must occur once: {before!r}; found {count}")
    return text.replace(before, after, 1)


def run_text(plan: str, roadmap: str, should_pass: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="a1-compatibility-plan-") as temporary:
        root = Path(temporary)
        plan_path = root / "docs/roadmap/a1-shared-compatibility-foundation.toml"
        plan_path.parent.mkdir(parents=True)
        plan_path.write_text(plan, encoding="utf-8")
        (plan_path.parent / "platform-core-adoption-milestones.toml").write_text(
            roadmap, encoding="utf-8"
        )
        result = subprocess.run(
            [str(CHECKER), str(root)], capture_output=True, text=True, check=False
        )
        if (result.returncode == 0) != should_pass:
            raise AssertionError(
                f"expected pass={should_pass}, got {result.returncode}\n"
                f"stdout={result.stdout}\nstderr={result.stderr}"
            )


def main() -> None:
    plan = PLAN.read_text(encoding="utf-8")
    roadmap = ROADMAP.read_text(encoding="utf-8")
    run_text(plan, roadmap, True)
    run_text(replace_once(plan, "parent_issue     = 504", "parent_issue     = 420"), roadmap, False)
    run_text(replace_once(plan, "root_issues      = [ 420, 505 ]", "root_issues      = [ 420 ]"), roadmap, False)
    run_text(
        replace_once(
            plan,
            'id              = "quality-routing"\nissue           = 501\ntitle           = "risk-routed quality evidence declarations"\nstate           = "planned"\nblocked_by      = [ 420 ]',
            'id              = "quality-routing"\nissue           = 501\ntitle           = "risk-routed quality evidence declarations"\nstate           = "planned"\nblocked_by      = [  ]',
        ),
        roadmap,
        False,
    )
    run_text(
        replace_once(
            plan,
            "blocked_by  = [ 420, 422, 501, 505 ]\noutcome",
            "blocked_by  = [ 420, 422, 501 ]\noutcome",
        ),
        roadmap,
        False,
    )
    run_text(
        plan,
        replace_once(
            roadmap,
            "owner_issue       = 504\nsupporting_issues = [ 420, 422, 501, 505 ]",
            "owner_issue       = 420\nsupporting_issues = [ 501 ]",
        ),
        False,
    )
    print("a1-compatibility-plan-tests: mutation suite passed")


if __name__ == "__main__":
    main()
