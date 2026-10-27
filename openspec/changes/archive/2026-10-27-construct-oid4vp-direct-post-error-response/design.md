# Design

## Explicit policy transition

```text
ValidatedAuthorizationRequest        explicit wallet policy
  - validated HTTPS response_uri   + DirectPostErrorCode
  - optional exact state           + optional bounded description/URI
                 \                 /
                  v               v
             DirectPostErrorResponse
               - POST target
               - form media type
               - bounded zeroizing form body
```

The request is consumed so its target and state cannot be rebound. The caller
selects a protocol error explicitly; no conversion from `Oid4vpError` exists.

## Typed protocol values

`DirectPostErrorCode` is a fieldless closed enum whose `as_str` returns the
case-sensitive RFC/Final value. `DirectPostErrorDescription` owns a bounded
non-empty NQSCHAR string. `DirectPostErrorUri` owns a bounded syntactically valid
URI-reference. Their diagnostics reveal only code kind, byte length, or presence.

`DirectPostErrorDetails` groups optional developer fields. Its constructor
validates each input against `DirectPostErrorResponseLimits`; callers can use an
empty value for the privacy-minimal error response. No value is treated as safe
for UI display, resolution, or network access.

## Deterministic envelope

The constructor reuses the in-crate form-length and encoding implementation.
It computes the complete encoded length using checked arithmetic, rejects an
oversized body before allocation, and writes `error`, `error_description`,
`error_uri`, and `state` in that order when present. The result exposes method,
media type, lengths/presence, and explicitly sensitive target/body accessors.

The success and error constructors are separate consuming methods, so one
envelope cannot contain both `vp_token` and `error`. Common private form helpers
are data-agnostic and do not create a generic public OAuth abstraction.

## Verification

Clean-room official vectors cover the Final direct-post example and every
closed code. Negative/boundary cases cover empty/invalid NQSCHAR, malformed or
oversized URI-reference, exact body limit, exact state retention, field order,
encoding and redaction. Property tests round-trip arbitrary accepted values,
fuzzing targets constructors/form output, and benchmarks measure bounded build
cost. Existing success E2E tests remain green and one request-lineage E2E test
proves the mutually exclusive error branch.

## Slice-size note

Production code remains inside `identus-oid4vp`; additional paths are required
specification, ADR, vectors and quality/knowledge records. HTTP, protected
responses, privacy policy and consumer adoption remain independent successors.
