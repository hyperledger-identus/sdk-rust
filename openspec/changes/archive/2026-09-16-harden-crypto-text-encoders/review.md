# Exact-diff security, API, and architecture review

- **Review date:** 2026-09-17
- **Issue:** #298
- **Develop base:** `508917b0416147668b9d12949eec987f0b82946b`
- **Reviewed implementation head:** `c36e7ca688d0e5b720ac6e40361dce92183c0a27`
- **Result:** passed with no unresolved blocker

## Scope reviewed

The review re-read the complete 26-path exact diff, the public API rendering,
both codec implementations, JWK migrations, compile-fail and boundary tests,
ADR 0129, the input-resource inventory, and the narrowed `SDK-LIM-007` text.
It also searched the workspace for remaining production calls to the removed
infallible constructors and verified every implementation commit signature.

## Findings

1. **Public reachability — accepted.** The generic infallible `From` surface is
   absent. The named method and four `TryFrom` ownership forms all delegate to
   the same checked constructor. Compile-fail tests prevent accidental return
   of the old source API.
2. **Resource arithmetic — accepted.** Hex accepts at most 2,048 raw bytes;
   unpadded Base64url accepts at most 3,072. Those values encode to exactly the
   existing 4,096-byte text ceiling. One-over inputs calculate 4,098 encoded
   bytes, reject before encoder allocation, and use saturating/checked length
   arithmetic for larger hostile lengths.
3. **Trusted internal path — accepted.** `encode_trusted` is crate-private and
   used only after text parsing has enforced the shared ceiling or for fixed
   32-byte JWK coordinates. No public unchecked constructor or serde bypass is
   introduced.
4. **Compatibility — accepted.** Canonical lowercase hex, unpadded Base64url,
   parsing, serialization, and JWK wire values are unchanged. The source break
   is explicit and appropriate for unpublished `0.0.0` crates; ADR 0129 gives
   a direct fallible migration.
5. **Error and privacy contract — accepted.** Oversize failures remain
   `Error::KeyParsing`, carry only codec name, limit, and observed length, and
   bridge to the static `crypto.key_parsing` public error. Tests prove sentinel
   input is not reflected.
6. **Feature and target shape — accepted after remediation.** The first slow
   run exposed that the integration test imported codec types under
   `--no-default-features`. The test now has an explicit `hex` plus `base64`
   feature gate; the minimal lane and strict minimal Clippy pass without
   weakening the all-feature boundary evidence.
7. **Governance precision — accepted.** The removed compatibility row is
   absorbed into the enforced crypto boundary. Only the codec clause is removed
   from `SDK-LIM-007`; caller preallocation, native JSON cleanup, delegated
   work, direct JOSE enum, and DID Multihash obligations remain.
8. **Dependency and unsafe boundary — accepted.** No dependency, feature,
   compiler, lockfile, build script, network, storage, FFI, or unsafe-code
   change exists.
9. **Exported limit documentation — accepted after hosted review.** The first
   automated review correctly found that `MAX_CRYPTO_TEXT_BYTES` still
   described the removed parser-only/infallible-encoding contract. Its rustdoc
   now states the shared retained-text ceiling for parsing and fallible byte
   encoding while preserving the caller-allocation caveat.

## Decomposition decision

The diff exceeds the preferred changed-file count because one indivisible
public API migration carries its planning record, ADR, machine inventory,
canonical constraint text, API snapshot, implementation, and negative evidence
atomically. Splitting those artifacts would create either an undocumented API
break or a false governance claim. The production code remains two cohesive
codec modules plus four fixed-width JWK call-site edits; no further feature
decomposition is warranted.

## Residual limitations

- Callers allocate their byte input before entering the SDK boundary.
- This change does not widen the supported codec profiles or authorize a
  registry release.
- The independent JOSE enum and DID Multihash exceptions remain tracked under
  `SDK-LIM-007`.

## Decision

The implementation is bounded, cohesive, redaction-safe, reversible, and
consistent with the unpublished compatibility policy. No unresolved security,
API, architecture, portability, dependency, or governance finding remains.
