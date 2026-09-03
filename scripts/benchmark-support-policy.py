#!/usr/bin/env python3
"""Measure support-policy validation without turning timing into a promise."""

from __future__ import annotations

import argparse
import importlib.util
import json
import platform
import statistics
import subprocess
import sys
import time
from pathlib import Path
from types import ModuleType
from typing import Any

MINIMUM_SAMPLES = 20
MATERIAL_REGRESSION_FACTOR = 2.0
MATERIAL_REGRESSION_ALLOWANCE_MS = 5.0


def load_validator(root: Path) -> ModuleType:
    checker = root / "scripts/check-support-policy.py"
    name = f"sdk_support_policy_{abs(hash(checker))}"
    spec = importlib.util.spec_from_file_location(name, checker)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {checker}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def percentile(values: list[float], percentile_value: float) -> float:
    ordered = sorted(values)
    rank = max(0, min(len(ordered) - 1, int((len(ordered) - 1) * percentile_value)))
    return ordered[rank]


def git_revision(root: Path) -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else "unknown"


def measure(root: Path, samples: int) -> dict[str, Any]:
    validator = load_validator(root)
    failures = validator.validate(root)
    if failures:
        raise RuntimeError(f"warm-up validation failed for {root}: {failures}")

    warm: list[float] = []
    for _ in range(samples):
        start = time.perf_counter()
        failures = validator.validate(root)
        warm.append((time.perf_counter() - start) * 1_000)
        if failures:
            raise RuntimeError(f"warm validation failed for {root}: {failures}")

    checker = root / "scripts/check-support-policy.py"
    process_cold: list[float] = []
    for _ in range(samples):
        start = time.perf_counter()
        result = subprocess.run(
            [sys.executable, str(checker), str(root)],
            check=False,
            capture_output=True,
            text=True,
        )
        process_cold.append((time.perf_counter() - start) * 1_000)
        if result.returncode != 0:
            raise RuntimeError(
                f"process-cold validation failed for {root}: {result.stderr}"
            )

    return {
        "root": str(root),
        "revision": git_revision(root),
        "samples": samples,
        "warm_p50_ms": round(statistics.median(warm), 3),
        "warm_p95_ms": round(percentile(warm, 0.95), 3),
        "process_cold_p50_ms": round(statistics.median(process_cold), 3),
        "process_cold_p95_ms": round(percentile(process_cold, 0.95), 3),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--root", type=Path, default=Path(__file__).resolve().parents[1]
    )
    parser.add_argument("--baseline-root", type=Path)
    parser.add_argument("--samples", type=int, default=MINIMUM_SAMPLES)
    args = parser.parse_args()
    if args.samples < MINIMUM_SAMPLES:
        parser.error(f"--samples must be at least {MINIMUM_SAMPLES}")

    current = measure(args.root.resolve(), args.samples)
    report: dict[str, Any] = {
        "schema_version": 1,
        "platform": platform.platform(),
        "python": platform.python_version(),
        "current": current,
    }
    if args.baseline_root is not None:
        baseline = measure(args.baseline_root.resolve(), args.samples)
        comparisons: dict[str, float] = {}
        regressions: list[str] = []
        for metric in ("warm_p95_ms", "process_cold_p95_ms"):
            current_value = float(current[metric])
            baseline_value = float(baseline[metric])
            comparisons[f"{metric}_ratio"] = round(current_value / baseline_value, 3)
            ceiling = (
                baseline_value * MATERIAL_REGRESSION_FACTOR
                + MATERIAL_REGRESSION_ALLOWANCE_MS
            )
            if current_value > ceiling:
                regressions.append(
                    f"{metric} {current_value:.3f} ms exceeds material-regression ceiling {ceiling:.3f} ms"
                )
        report["baseline"] = baseline
        report["comparison"] = comparisons
        report["material_regressions"] = regressions
        print(json.dumps(report, sort_keys=True))
        if regressions:
            for regression in regressions:
                print(f"support-policy-benchmark: {regression}", file=sys.stderr)
            return 1
    else:
        print(json.dumps(report, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
