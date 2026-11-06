# Design

## Exact rebaseline receipt

Add one closed TOML receipt beside the DID candidate descriptor. It binds issue
#582, protected product-source revision, cargo-public-api version, old and new
baseline SHA-256 values, the unchanged resolver baseline, an additive-only
classification, zero removals, and the exact allowed added API lines.

Extend the existing release-candidate checker and mutation suite to reject a
missing, malformed, symlinked, oversized, stale, or scope-expanded receipt.
The checker recomputes both tracked baseline hashes and requires the addition
set to describe only the bounded JSON-wire surface.

## Baseline update

Generate the current API through the pinned candidate toolchain in deliberate
review mode. Copy the generated `identus-did` API text byte-for-byte; do not
edit or normalize it manually. Leave the resolver HTTP baseline untouched.
Update ADR/readiness knowledge with the exact classification and claim
boundary.

## Qualification

Run ordinary candidate preparation without `--initialize-api`. It must
reproduce both committed baselines, deterministic archives, patched closure,
SBOMs, source/license policy, and bounded receipt. Run primary/MSRV matrix
lanes, candidate mutations, full factory/site/Nix gates, and independent
exact-diff review.

Failure remains closed. Any removed/changed API line, extra addition,
unexpected resolver change, product-source edit, dependency drift, or normal
candidate mismatch blocks #388 rather than widening the receipt.
