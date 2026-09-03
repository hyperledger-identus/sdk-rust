# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-03
- **Issue:** #35, child of #5 / `IDR-005`
- **Develop base:** `267b435ece23d935fc4a4946df0b496c91466e66`
- **Started:** `2026-09-03T09:44:39Z`
- **Result:** contract is implementable with no unresolved blocker

## Ownership and provenance findings

Generic DID/DID URL lexical assurance belongs with the generic parser in
sdk-rust. Midnight's existing property tests generate method-specific off-chain
state and therefore remain in midnight-identity. NeoPRISM, Lace and Oxid supply
consumer-shaped identifiers only; Apollo has no DID parser. A new shared crate
or donor extraction would add coupling without reusable behavior. No donor code
or fixture is needed.

The fixed W3C Recommendation remains the grammar authority. Current W3C editor
and test-suite revisions are provenance/drift evidence, not moving normative
inputs. The repository-pinned nightly and Nixpkgs revision resolve cargo-fuzz
without an ambient install; exact `libfuzzer-sys` prevents silent runner-runtime
drift.

## Security and resource findings

Arbitrary bytes are appropriate input, but non-UTF-8 cannot reach an API that
accepts `&str` and should not be decoded lossily. An 8 KiB generator limit
crosses both production caps. Five-second per-input and 1 GiB process ceilings
contain pathological harness/compiler behavior while remaining generous for a
linear 4 KiB parser.

Accepted-value assertions must use public accessors, not internal offsets, so
the campaign protects the contract consumers actually receive. Pointer-range
checks plus exact delimiter-aware reconstruction expose stale or invalid ranges
without expanding the production API. Serde and owned/native constructors are
worth fuzzing because each is a public validation boundary.

## Automation and compatibility findings

Fixed-run PR smoke with one seed/worker is a reproducible regression search;
time-boxed scheduled soak is intentionally nondeterministic evidence. Keeping
those modes separate protects normal PR latency. Crash artifacts must upload
only on failure and be treated as untrusted, potentially sensitive data.

The independent fuzz workspace leaves all published dependencies, features,
public types, wire shapes, MSRV and cross-target production builds unchanged.
Linux sanitizer CI is sufficient for this bounded slice; the existing Nix
matrix retains cross-platform authority. No downstream edit, release,
publication, unsafe production code, repository setting, or `main` change is
justified.

## Implementation review

Pending after the production diff is frozen and all local gates pass.
