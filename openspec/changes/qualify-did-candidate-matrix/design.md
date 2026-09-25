# Design

## Descriptor-owned matrix

Extend `did-candidate.toml` with closed native-host and portable-target records.
Each record names exact packages, compiler classes, evidence tier, operation,
and limitation. The existing global support policy remains authoritative for
compiler values and broad workspace eligibility; the candidate descriptor may
only narrow it.

## Reuse the candidate renderer

Add a matrix execution mode to `prepare-did-candidate.py` rather than creating
a second staging implementation. It renders the same isolated
`0.1.0-rc.1` workspace used by archive/API/SBOM evidence, generates and hashes
one lockfile, then executes descriptor-selected commands. The normal candidate
path and its existing hashes remain unchanged.

Primary lanes run every package profile through host tests. MSRV lanes compile
the same profiles without pretending to be runtime evidence. Each lane also
executes only portable targets assigned to its native runner: Linux owns WASM
and Android ARM64; macOS owns iOS ARM64. Unsupported HTTP target rows are
recorded from policy, not executed.

## Pinned execution apps

Add two small Nix applications over the existing exact primary and MSRV
toolchains. Both call the same matrix mode and inject only their closed compiler
class. They do not add a toolchain, dependency, target, credential, or package.

## Slow evidence

Add one native host matrix job to the existing weekly/manual slow workflow.
Each host invokes both Nix apps and uploads two lane receipts. The final
evidence job downloads exactly four receipts and calls aggregate mode. The
aggregate matrix receipt is retained beside the existing slow-run receipt and
the slow-run job map binds the matrix job outcome.

No pull-request trigger, required status, automatic dispatch, or privileged
environment is added. The normal PR still runs only `fast` plus policy/file
checks.

## Validation and threat model

Descriptor validation rejects unknown/missing fields, duplicates, compiler or
host drift, candidate packages outside the train, portable package broadening,
wrong operation/tier, and missing limitations. Lane validation rejects dirty
or wrong source, ambient compiler/host mismatch, malformed/oversized evidence,
unexpected commands/outcomes, and incomplete entries. Aggregation rejects
missing/duplicate lanes, source/lock disagreement, unsupported-target
overclaim, failure, and extra files.

The command allowlist remains local Cargo/Git/Python only. Scratch paths stay
outside the repository and output publication is atomic. Receipts never record
command output or environment contents.

## Verification and rollback

Run focused mutations and synthetic receipts, both local macOS compiler lanes,
Nix app evaluation/build, format/lint/factory/OpenSpec gates, and local review.
The hosted PR proves the unchanged fast line. Rollback removes the matrix
records/apps/workflow/receipt path together and returns M5 to unqualified state.
