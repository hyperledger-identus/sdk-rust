# Review: bounded OID4VCI Credential Offer grants

## Pre-implementation semantic review — 2026-09-06

### Scope and architecture

- **Pass:** the owner remains the chain-neutral, unpublished
  `identus-oid4vci` crate; the transition consumes a validated core offer and
  does not activate the quarantined umbrella crate or mutate a consumer.
- **Pass:** typed known grants do not choose or execute a flow. Metadata,
  authorization, token exchange, replay, trust, consent, storage, chain
  behavior, FFI, adoption, publication, and release remain excluded.
- **Pass:** unknown grant names/members and exact JSON remain lossless, so this
  bounded slice does not close the protocol's extension point.

### Standards and compatibility

- **Pass:** the contract matches OpenID4VCI Final section 4.1.1: absent/empty
  grants defer selection, multiple grants remain caller-selected, both known
  grant values are objects, and Pre-Authorized Code is required in its grant.
- **Pass:** a present `tx_code` is an object even when empty; optional
  `numeric`/`text`, integer length, and 300-character description rules are
  represented without confusing requirements with the later user input.
- **Pass:** RFC 8414 issuer syntax is applied to Authorization Server hints,
  while the Final metadata membership and multiple-server rules are deferred
  until metadata is available.
- **Pass:** Lace's `tx_code: null` stays visible as legacy-negative evidence;
  no released compatibility exists and no consumer is silently widened.

### Security and privacy

- **Pass:** issuer state and Pre-Authorized Code are attacker-controlled and
  bearer-adjacent. Positive limits, zeroizing ownership, explicit accessors,
  and redacted diagnostics address allocation and accidental-log threats.
- **Pass:** validation makes no freshness, single-use, replay, phishing,
  endpoint agreement, or trust claim. Section 13.6 mitigations remain future
  orchestration/issuer responsibilities.
- **Pass:** known-grant, Transaction Code, and scalar/integer type confusion
  fail before a grant-validated public value exists.

### Resource and portability review

- **Pass:** separate positive limits avoid breaking transport/core limit APIs;
  300 Unicode scalar values and 1,200 UTF-8 bytes are independently enforced.
- **Pass:** the selective scanner remains inside existing transport byte,
  depth, node, and duplicate-member ceilings and avoids a generic value tree.
- **Pass:** the dependency cone remains unchanged; no async, HTTP, crypto,
  runtime, platform, chain, or product dependency enters.

### Provenance and isolation

- **Pass:** no donor source or fixture is copied. Official and independently
  reconstructed values suffice for this grant-shape slice.
- **Pass:** Oxid and Lace revisions/status/digests are recorded as read-only
  evidence; other consumer/donor dirty state is pre-existing and untouched.

## Decision

The contract is semantically ready for implementation. No blocking finding or
protected decision remains. Structural factory validation is separate evidence.

## Post-implementation review

### Candidate and method — 2026-09-06

- Reviewed implementation commit:
  `23fcbd872fa49d4a16ecfcc623098eda6a55830b` against
  `origin/develop@bbfbbd77a68a619a616346aa9d3f31285b6effb3`.
- Re-read the complete production/test/specification diff after implementation;
  checked every public accessor and error mapping, the third bounded scanner
  pass, integer/string/container boundaries, exact-JSON retention, dependency
  graph, diagnostic surfaces, and the Final sections cited by issue #115.
- Confirmed the implementation does not interpret extensions or select a flow,
  and that later metadata matching remains impossible to mistake for completed
  validation through the API or documentation.

### Findings resolved before readiness

1. **Extension grant object shape:** the first implementation skipped unknown
   grant values as arbitrary JSON. Final section 4.1.1 says every grant value is
   an object. The scanner now checks `{` before losslessly skipping an unknown
   value, and a scalar-extension negative test prevents regression.
2. **Fail-closed typed conversion:** the first conversion treated a scanner
   invariant for Transaction Code input modes as infallible. The conversion now
   returns `Result`, rejects an unexpected value with the static mode error, and
   contains no panic or unreachable assertion on untrusted-data paths.

### Final assessment

- **Architecture/API — pass:** the consuming transition prevents a caller from
  confusing core validation with grant validation; both known alternatives and
  absent/empty/unknown states remain representable. Additive unpublished types
  preserve the existing dependency cone and chain-neutral boundary.
- **Standards/compatibility — pass:** known grant and Transaction Code shapes,
  empty-object semantics, default input mode, integer rules, 300-character
  ceiling, multiple grants, and RFC 8414 identifier syntax match the frozen
  contract. Deliberate positive/non-empty policy is explicit rather than an
  accidental claim of protocol flow completion.
- **Security/privacy — pass:** bearer-adjacent strings are bounded, zeroizing,
  non-serializing and redacted; exact raw JSON remains behind an explicit
  accessor already established by transport validation. Errors remain static
  and do not retain parser causes or caller-controlled content.
- **Resource/portability — pass:** all additional allocations are bounded by
  transport bytes/nodes or positive grant limits; checked integer accumulation
  is architecture-safe, and no runtime, network, platform, crypto, or chain
  dependency was introduced.
- **Tests/evidence — pass:** official and independently reconstructed positives,
  strict type and exact-boundary negatives, transport-path equivalence,
  extensions, and diagnostic canaries exercise the complete new contract.

No unresolved blocker, warning, protected decision, or follow-up is hidden in
this slice. Hosted exact-head CI and review remain delivery gates, not local
semantic findings.
