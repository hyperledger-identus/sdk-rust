# Design

## Immutable published origin

Keep `docs/release/identus-crypto-0.1.0-rc.1.api.txt` byte-identical and add its
SHA-256 plus `published-origin` status to the crypto candidate descriptor.
Repository checks reject drift or any attempt to reinitialize that origin.

## Compatibility comparison

Generate current API through the pinned existing toolchain. Split baseline and
current output into line multisets. Subtract current counts from baseline counts:
any positive result is a removed/changed occurrence and fails qualification.
The reverse subtraction is a compatible additive set.

Write the current API text and a deterministic JSON report containing schema,
classification, pinned baseline/current digests, and sorted added/removed line
multisets. Cap output size under the existing candidate evidence boundary. The
present report must show twelve additions: four method signatures rendered at
three re-export views, and zero removals.

## Qualification and regression

Ordinary candidate preparation must retain the published baseline while also
proving deterministic archives, required packaged fixtures, closure profiles,
the packaged all-feature test suite, semver checks, SBOMs, source policy, and
the compatibility report. Unit/mutation evidence proves additions pass,
removals—including duplicate-count loss—fail, the baseline digest is fixed, and
published-origin initialization is denied.

No product-source or downstream change belongs in this slice.
