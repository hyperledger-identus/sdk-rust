# Design

## Evidence boundaries

The existing `rust-audit` check remains the authoritative vulnerability gate.
It runs only after a deterministic CVSS 4.0 parser probe and invokes the real
lockfile scan with the pinned database, `--no-fetch`, `--no-yanked`, and JSON
output. The result adapter emits one stable summary and rejects malformed or
ambiguous tool output.

The evidence model has four relevant outcomes:

| Outcome | Gate behavior |
| --- | --- |
| Advisory success | pass with pinned tool/database identity and zero matching vulnerabilities |
| Known vulnerability | fail with a stable vulnerability result |
| Incompatible tool | fail the CVSS 4.0 probe before dependency evidence is accepted |
| Yank unavailable | record separately and do not perform noisy per-package lookup |

## Compatibility probe

A repository fixture contains a minimal valid RustSec database with a CVSS 4.0
advisory and a non-vulnerable Cargo lockfile. The same pinned `cargo-audit`
binary and `--no-yanked` mode used by the real scan must parse it successfully.
Mutation tests replace the command/result boundary to prove the incompatible
tool and vulnerability classifications deterministically; they do not depend
on retaining an obsolete vulnerable executable.

## Nix integration

The Nix check asserts the reviewed `cargo-audit` version and supplies the
explicit argument override to Crane. Repository-owned checking code and
fixtures are kept within the derivation input. A structured result is retained
in the Nix output and a concise status is printed once. No network, host Cargo
cache, or registry index is mounted into the sandbox.

## Yank evidence

Yank state is not derivable from RustSec advisories. Because the Nix lane lacks
an authoritative registry index, its structured result says `unavailable` with
the reason `registry-index-not-provided`. A future gate may replace only that
field after it defines immutable source identity, freshness, failure policy,
and release semantics.

## Verification

Focused tests cover all four outcomes and ensure the real pinned database
contains/parses the named CVSS 4.0 regression record. Nix evaluation/build,
factory structure, formatting, text/TOML/Nix lint, and the protected fast gate
must pass. Discussion #399 receives the final evidence and remaining yank
limitation.
