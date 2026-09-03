# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-03
- **Issue:** #41, child of #5 / `IDR-005`
- **Develop base:** `1e2e64265e048c5f7bfe06ffcba39de881611517`
- **Result:** contract is implementable with no unresolved blocker

## Standards and compatibility findings

The issue correctly names the 28 August Candidate Recommendation Draft. The
6 August Snapshot is the stronger compatibility pin; comparing editor source
from the snapshot publication through `71a5005` finds only non-normative
implementation-suite guidance and publication configuration. The existing
nine error URLs and result shapes therefore remain compatible.

The current four-digit datetime parser is a valid common subset but not the
complete intersection required by the draft's XML Schema 1.1 reference. A
bounded string validator can admit astronomical zero, negative/extended years,
and `24:00:00Z` without a datetime dependency or lossy conversion. Requiring
`Z` and rejecting fractions follows the additional DID Resolution constraints.

## Security and resource findings

Scanning after `serde_json::Value` or typed deserialization would be too late:
duplicates have already collapsed. The #38 streaming scanner is suitable when
the result byte cap is checked first and its complete-input traversal precedes
typed parsing. Result-specific static error reasons preserve diagnostic
clarity without carrying attacker strings.

The scanner's 64-depth, 16,384-node, 128-member, and 128-KiB-live-name limits
bound its object-local decoded-name sets. Existing typed semantic limits remain
tighter for open data. The double traversal is an acceptable availability
trade-off and receives fixed-work, no-threshold release evidence.

## Conformance, provenance, and boundary findings

The official implementation-report suite is BSD-3-Clause and currently tests
HTTP resolution rather than publishing a reusable result/dereferencing fixture
corpus. Its portable result assertions can be recreated as original Rust
vectors; copying JavaScript or claiming dereferencing-suite coverage would be
incorrect. Cargo-fuzz remains #35, while deterministic property matrices make
this result slice reproducible across the supported target matrix.

Arbitrary native bytes cannot be added losslessly to the current JSON value
without selecting an encoding. A future binding-specific media-type plus
bounded-byte value is the reversible boundary; implicit UTF-8/base64 and
network retrieval stay rejected. No chain/product dependency, unsafe code,
consumer mutation, release, publication, repository administration, or `main`
change is justified.

## Exact-head implementation review

- **Date:** 2026-09-03
- **Reviewed head:** `cd8538877c44515fa992520c3f48b79eb25bc012`
- **Reviewer:** local independent second pass after implementation was frozen
- **Result:** no semantic, security, API, or delivery blocker found

The public change is additive: four static `ResolutionError` reasons and five
documented resource constants are added to an already non-exhaustive error
surface. Existing constructors and serde representations are unchanged. The
two explicit raw result entry points now share the already-reviewed streaming
scanner, run it only after the 512 KiB byte check, and map every scanner outcome
to static text. No caller bytes, names, values, or positions enter diagnostics.

The datetime validator was reviewed against the XML Schema 1.1 lexical regular
expression and DID Resolution's stricter UTC/whole-second rule. It accepts the
optional negative sign, four-or-more-digit astronomical years without
unnecessary leading zeroes, proleptic-Gregorian dates, and `24:00:00Z`. Its
modulo-400 fold cannot overflow, and the 128-byte limit bounds time and memory.
Offsets, fractions, missing `Z`, non-ASCII input, malformed dates, and invalid
end-of-day values fail closed.

The test diff covers escaped-equivalent duplicate names, object-local reuse,
every scanner ceiling, complete-input parsing, redaction, all result-state
combinations, all nine standard error URLs, deterministic scalar/extension
matrices, the portable W3C suite assertions, and a no-threshold release
diagnostic. Production code adds no dependency, unsafe block, runtime, network,
method, chain, custody, trust, or product policy. The exact base-to-head diff is
format-clean, and normal dependency topology remains unchanged.

## Downstream immutability review

Post-implementation receipts equal the pre-implementation receipts:

| Repository | HEAD / branch | Preserved pre-existing status |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` / `main` | clean |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` / `develop` | modified `third_party/midnight-did` submodule |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` / `main` | untracked `.pi-subagents/`, `.pi/`, `tmp/` |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` / `integration` | untracked `.claude/`, `.pi/taskflows/` |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` / `main` | modified `secp256k1-kmp/native/secp256k1` submodule |

No downstream file, branch, index, working-tree state, submodule state, or
remote was changed.
