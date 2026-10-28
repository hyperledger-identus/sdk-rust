# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/538
Constraint blockers: none

## Existing entries affected

ADR 0181 and `jose-jwe-encryption-port` already require an SDK-owned facade and
a separately qualified private engine. The OpenID4VP differential declaration
owned by #535 remains required debt until a production adapter runs the
independent oracle in repository evidence.

## Introduced or changed constraints

Qualification must use exact released versions and record the selected feature
set, MSRV, license, direct and resolved dependency cone, unsafe/native code,
RustSec result, public-recipient API, redaction boundary, and host/WASM/iOS/
Android compilation. An adapter may be adopted only when every mandatory gate
passes without advisory suppression or workspace policy exceptions.

Independent evidence must encrypt with the verifier public P-256 key and
decrypt with the corresponding private key. It must compare algorithms, `kid`,
protected-header shape and plaintext rather than randomized ciphertext bytes.
The oracle remains outside the workspace graph unless its engine is adopted.

## Introduced or changed limitations

This slice does not claim a built-in production encrypter. The externally
executed oracle proves engine capability and portable compilation only; it does
not make that engine a supported SDK dependency. Browser entropy execution,
consumer bundlers and product integration remain future adoption evidence.

## Consumer and product impact

None. The public SDK API, wire contract, lockfile and supported algorithm set
remain unchanged. Consumers still inject a conforming JWE encrypter.

## Activation and rollback

Activation requires exact #538 preflight, ADR 0192, durable candidate evidence,
the bounded external reproduction route, updated knowledge/evidence records,
local validation, independent CLI review, green protected CI and an issue-linked
PR. Rollback removes behavior-neutral evidence only.

## Evidence

Issue #538, ADRs 0181 and 0192, exact crates.io source packages, the RustSec
advisory database, the external round-trip route and repository factory receipts
are the evidence authorities.
