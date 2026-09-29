# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@eca0218779cb960ae4e4b982095fd878ebf5371a`,
`validate_datetime` performs this bounded transition:

1. require 20 through 128 ASCII bytes and a terminal `Z`;
2. accept an optional leading minus, find the first year delimiter, and require
   at least four year digits with no leading zero for extended years;
3. require the exact 16-byte `-MM-DDThh:mm:ssZ` tail and digits in every
   non-separator position;
4. project two-digit fields without allocation and fold the arbitrarily long
   bounded year to modulo 400;
5. derive the proleptic Gregorian month length and reject day zero or overflow;
6. accept `00:00:00` through `23:59:59`, plus only exact `24:00:00`; and
7. project every failure to the static redaction-safe
   `InvalidResolution(InvalidDateTime)` error.

`DidResolutionDateTime::parse` allocates only after successful borrowed
validation. `try_new` retains the caller's accepted allocation. `FromStr`,
`TryFrom<String>`, and Serde delegate to those paths. Existing deterministic
tests cover positive, astronomical-zero, negative and extended years; leap
centuries; bounds; normal and end-of-day times; malformed structure; constructor
equivalence; and Serde round trips. A compact exhaustive separator/digit and
calendar/time boundary matrix will bind the grammar before movement.

The canonical signal is 71 SLOC / cognitive 17 / cyclomatic 40. The containing
module is below 1,000 authored nonblank lines, and no production module exceeds
that threshold.

## Normative sources

Issue #456, Discussion #399, ADR 0017, ADR 0115, the W3C DID Resolution
baseline and XML Schema 1.1 profile already pinned by issue #41, and the
canonical DID core, code-health, resource-boundary, and spec-driven-delivery
contracts are authoritative. This slice changes no normative profile or wire
grammar, so no new external standard revision is adopted.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One private lexical parser returning one private calendar value | `adopt` | It gives structure and semantic validation distinct owners without exposing state or allocating. | Characterization cannot be retained or a replacement helper crosses a threshold. |
| Keep the validator intact and document an exception | `not-adopt` | Correctness outranks metrics if a private representation obscures the bounded grammar. | Stop/go review rejects decomposition. |
| One helper per field or conditional | `not-adopt` | Mechanical forwarding displaces metrics and fragments one grammar. | Never as a metric-only technique. |
| Adopt `time`, `chrono`, or `jiff` | `not-adopt` | A dependency would expand the cone and does not by itself guarantee exact XML Schema extended-year spelling, 128-byte bounds, allocation retention, or `24:00:00`. | A separate API/compatibility issue demonstrates the exact language and a justified cross-crate need. |
| Normalize to a timestamp or canonical text | `not-adopt` | It would lose exact spelling and cannot represent the existing bounded extended-year domain uniformly. | A separately approved wire-breaking profile requires normalization. |

## Compatibility and dependency evidence

The crate-root type, constructors, traits, exact retained string, error kind,
serialization, metadata/query consumers, constants, manifests, lockfile,
features, MSRV, and targets remain unchanged. No normal or development
dependency is added.

## Security, privacy and maintenance evidence

Input remains borrowed, ASCII-only, and bounded to 128 bytes before scanning.
Every scan is linear in that bound; fixed-position projection is allocation-
free; the modulo fold cannot overflow its `u16` accumulator. Errors remain
static and do not format input. The private value contains only small numeric
fields and cannot escape the module. No unsafe code, recursion, callback,
trait object, ambient I/O, clock, locale, timezone, synchronization, or panic
path is introduced.

## Rejected or deferred candidates

Fractional seconds, offsets, leap seconds, timezone lookup, normalization,
date arithmetic, public parsed fields, generic datetime abstractions, new
dependencies, and media/version validator refactors are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private parsed value cannot preserve the exact lexical language and allocation
behavior without a helper swarm, production remains unchanged and the signal
receives a measured exception.

## Evidence commands

Planning inspected the exact protected base, issue #456, validator, all
datetime tests and callers, ADR 0017, the canonical DID requirement, public API
inventory, dependency cone, and code-health report. Before production edits,
run the focused DID suite and add the boundary characterization. Afterward run
focused/workspace tests, strict Clippy/format/docs, public/source/code-health/
factory checks, portable targets, relevant Nix gates, distinct exact-diff
review, and protected exact-head CI.
