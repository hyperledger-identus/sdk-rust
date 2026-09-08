# Design

## Context

One path type feeds three derivation implementations. Text parsing currently
creates an intermediate vector of borrowed segments, while Cardano accepts the
typed path directly. The stateful BIP-32 and SLIP-0010 types additionally own
depth metadata. A coherent solution therefore needs one path budget plus a
depth preflight at each cryptographic boundary.

## Decisions

### Shared public budgets

The derivation module exports `MAX_DERIVATION_PATH_BYTES = 4_096`,
`MAX_DERIVATION_PATH_AXES = 255`, `MIN_HD_SEED_BYTES = 16` and
`MAX_HD_SEED_BYTES = 64`. Path limits are format/work policy; seed values come
from BIP-32 and SLIP-0010.

### Resource-first streaming parse

`from_path` checks `path.len()` first, obtains the root from a split iterator,
and pushes axes directly into one vector. It checks capacity before parsing
each segment. Existing root trimming/case behavior and axis syntax remain
unchanged for accepted input.

### Preflight complete work

`DerivationPath` exposes `len` and `is_empty` and an internal bound check.
String-derived HD keys receive an already bounded path, then verify current
depth plus path length before cloning/deriving. Individual `derive_child`
calls reject parents at depth 255 before HMAC. Cardano typed-path consumers
reject more than 255 axes before converting or deriving the first key.

### Preserve programmatic append compatibility

The existing `derive(&self, axis) -> Self` remains infallible. Changing it to a
`Result` would be a broad source break. It may create an oversized value, but
all cryptographic consumers fail before proportional cryptographic work. Its
documentation makes this caller-budgeted status explicit.

### Preserve stable errors and secrets

Every new rejection uses `Error::DerivationFailed`. No input or key content is
added to errors or formatting. Seed checking occurs before HMAC; path/depth
checking occurs before HMAC or curve operations. Existing zeroization remains.

## Compatibility

The constants and introspection methods are additive. Oversized or over-depth
input is newly rejected under issue #199. In-range derivation outputs, public
types, error enum, wire bytes, features, MSRV and dependency cone are unchanged.

## Alternatives rejected

A configurable limit, 32-axis wallet policy, dependency replacement and a
breaking fallible path builder do not belong in this focused security slice.

## Verification strategy

Unit tests cover text bytes, streaming precedence, 255/256 axes, seed endpoints
and child depth. Cardano integration tests cover exact/over typed path work.
Existing BIP-32, SLIP-0010, Apollo and Cardano vectors prove compatibility.
Feature-isolated and complete Nix gates prove portability and policy.
