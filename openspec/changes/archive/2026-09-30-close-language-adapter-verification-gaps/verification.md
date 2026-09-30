# Verification

## Local evidence

The implementation content at
`546602f588153b68e74609ecd4302665a6246fab` passed:

- `python3 scripts/check-language-adapter-mappings.py`
  - `4 canonical mappings passed`
- `python3 scripts/tests/language-adapter-mappings.py`
  - mutation suite passed
- `scripts/factory check`
  - `105 passed, 0 failed`
- `/nix/var/nix/profiles/default/bin/nix flake check`
  - all 38 compatible `aarch64-darwin` checks passed
  - `x86_64-linux` was reported as incompatible and remains hosted/scheduled
    evidence rather than a local emulation claim
- `git diff --check`
  - passed
- signed-commit verification
  - good signature and DCO trailer on every issue #510 commit

## Mutation coverage

Negative evidence covers closed and malformed tables, required seed identity,
provenance shapes, lifecycle states, licenses, consumer ownership, redaction,
selectors, field/error/loss/unsupported record invariants, invalid Rust API paths,
duplicated crate prefixes, stable-code/path confusion, broad version windows,
opposite and `both` directions inside one-way mappings, absent or malformed
losses, unsupported/loss mismatch, weakened byte ceilings, traversal, parent
and leaf symlinks, missing, duplicated, computed, invalid-UTF-8, and oversized
bound sources, plus unknown, ambiguous, wrong-capability, wrong-target, and
malformed vector catalogs. Every validator failure is asserted to omit a Python
traceback.

Additional verification mutations cover scalar error metadata before vector
outcome correlation, cross-crate bound substitution, a symlinked vector catalog,
and the exact invalid-UTF-8 diagnostic classification.

The final mutation set additionally covers DID-to-DID-URL same-crate bound
substitution, boolean schema versions, float issue numbers, duplicate field and
stable-error identities, and removed/transitional lifecycle contradictions.
Key semantic mutations additionally assert the exact bounded diagnostic rather
than accepting failure through an earlier unrelated rule.

## Final reviewed candidate

Independent verification at
`c6f520251fdc4f1e3840ac9226061098fbe119d1` found no blocking issue and
independently passed the checker, mutation suite, factory/factory-contract
gates, `git diff --check`, and signed/DCO history inspection. Hosted `fast`,
pull-request-policy, DCO, and file-hygiene checks also passed at that exact head.

Cross-catalog mutations also prove that successful vectors cannot evidence
error mappings and stable-error vectors cannot evidence value mappings.

Positive evidence covers the four canonical SDK-TS records, deterministic
checked-in rendering, repeated shared vector use, and an additive Swift record
whose bound is discovered in `crates/did/src/uri.rs` rather than a hardcoded DID
fixture path.

## Hosted evidence still required

The replacement PR must pass the exact-head `fast` lane and one fresh
independent discovery review before merge. Draft #509 remains non-mergeable and
will be closed as superseded only after the replacement candidate is ready.
