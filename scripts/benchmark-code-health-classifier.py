#!/usr/bin/env python3
"""Measure bounded near-limit classifier elapsed time and peak resident memory."""

from __future__ import annotations

import argparse
import json
import resource
import statistics
import subprocess
import sys
import time
from pathlib import Path


def canonical_json(value: object) -> str:
    return json.dumps(value, indent=2, sort_keys=True) + "\n"


def peak_rss_mib() -> float:
    raw = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    divisor = 1024 * 1024 if sys.platform == "darwin" else 1024
    return raw / divisor


def request(projection_nodes: int, module_edges: int) -> dict[str, object]:
    root = []
    sources = []
    for index in range(projection_nodes):
        root.append(f"#[cfg(test)] const TEST_{index}: usize = {index};\n")
    for index in range(module_edges):
        root.append(
            f'#[cfg(test)] #[path = "fixtures/f{index}.rs"] mod f{index};\n'
        )
        sources.append(
            {
                "path": f"crates/fixture/src/fixtures/f{index}.rs",
                "source": "pub fn fixture() {}\n",
            }
        )
    sources.append(
        {"path": "crates/fixture/src/lib.rs", "source": "".join(root)}
    )
    return {
        "protocol_version": 2,
        "sources": sources,
        "target_roots": ["crates/fixture/src/lib.rs"],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--binary", type=Path, default=Path("target/debug/code-health-classifier")
    )
    parser.add_argument(
        "--policy",
        type=Path,
        default=Path("docs/architecture/code-health-classifier-performance.json"),
    )
    args = parser.parse_args()
    policy = json.loads(args.policy.read_text(encoding="utf-8"))
    if set(policy) != {
        "max_elapsed_ms",
        "max_peak_rss_mib",
        "module_edges",
        "projection_nodes",
        "samples",
        "schema_version",
    } or policy["schema_version"] != 1:
        raise SystemExit("classifier benchmark policy is invalid")
    payload = json.dumps(
        request(policy["projection_nodes"], policy["module_edges"]),
        separators=(",", ":"),
    ).encode()
    elapsed = []
    for _ in range(policy["samples"]):
        started = time.monotonic_ns()
        completed = subprocess.run(
            [args.binary], input=payload, capture_output=True, check=False, timeout=30
        )
        elapsed.append((time.monotonic_ns() - started) / 1_000_000)
        if completed.returncode != 0:
            diagnostic = completed.stderr.decode(errors="replace").strip()
            raise SystemExit(f"classifier benchmark failed: {diagnostic}")
        response = json.loads(completed.stdout)
        if len(response.get("files", [])) != policy["module_edges"] + 1:
            raise SystemExit("classifier benchmark returned incomplete file coverage")
    ordered = sorted(elapsed)
    p95_index = max(0, (95 * len(ordered) + 99) // 100 - 1)
    result = {
        "elapsed_ms": {
            "max": round(max(elapsed), 3),
            "p50": round(statistics.median(elapsed), 3),
            "p95": round(ordered[p95_index], 3),
        },
        "module_edges": policy["module_edges"],
        "peak_rss_mib": round(peak_rss_mib(), 3),
        "projection_nodes": policy["projection_nodes"],
        "samples": policy["samples"],
        "schema_version": 1,
    }
    print(canonical_json(result), end="")
    if result["elapsed_ms"]["max"] > policy["max_elapsed_ms"]:
        raise SystemExit("classifier elapsed-time threshold exceeded")
    if result["peak_rss_mib"] > policy["max_peak_rss_mib"]:
        raise SystemExit("classifier peak-RSS threshold exceeded")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
