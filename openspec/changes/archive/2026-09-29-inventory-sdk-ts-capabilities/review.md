# Preimplementation review

Reviewer: independent semantic pass in a fresh reading context
Review date: 2026-09-29
Review status: ready
Blockers: none

## Findings

1. The change remains discovery-only and does not mutate SDK-TS or activate a
   binding, dependency, support promise, deprecation, or release.
2. SDK-TS is correctly treated as the primary current implementation-evidence
   baseline, not as a normative source that can override Final standards or
   stronger SDK-Rust behavior.
3. The target taxonomy removes historical module names without erasing source
   provenance or temporary language-SDK compatibility needs.
4. SD-JWT and AnonCreds deviations are decomposed by current profile and
   engine responsibility, so the plan cannot silently inherit stale donor
   dependencies.
5. The DID/DID URL canary is appropriately provisional, non-secret, narrow,
   opt-in, and separately gated.

No semantic blocker prevents planning preflight. Structural validation does
not constitute dependency, security, conformance, runtime, or consumer
approval.

# Post-implementation review

Reviewer: distinct semantic pass over the staged implementation and published
child issues
Review date: 2026-09-29
Review status: accepted
Blockers: none

## Evidence reviewed

- ADR 0169 and its precedence, naming, deviation, and format-normalization
  decisions;
- all 24 machine-readable capability records against the immutable SDK-TS
  8.1.4 source tree and package manifest;
- exact runtime, package-export, dependency, test-family, consumer, target
  owner, TypeScript owner, disposition, risk, and follow-up fields;
- global registry, roadmap, cross-SDK report, and blueprint terminology;
- issues #489 through #495 and the existing #418, #419, and #420 boundaries;
- the inventory checker, mutation suite, factory integration, strict OpenSpec
  validation, Markdown lint, and clean-diff evidence.

## Findings

1. Discovery order and decision authority are no longer conflated. SDK-TS
   identifies the current product surface, but the machine contract preserves
   the ADR 0169 authority order and the report explains how to apply it to each
   deviation.
2. No dependency or capability is activated by the inventory. SD-JWT,
   AnonCreds, and DIDComm donor engines are `replace-upstream` candidates with
   dedicated current-library assessments (#489, #490, #491).
3. The remaining ambiguous responsibilities are not hidden: peer/Prism method
   adapters, Presentation Exchange/DCQL, backup format, shared vectors, and
   protocol state have explicit issues or program owners.
4. Historical component labels occur only as source aliases, donor paths,
   immutable compatibility evidence, or explanatory prose. Target IDs and
   owners are responsibility-based, and the executable checker rejects a
   regression in target identity.
5. The canary is correctly isolated under #492: DID/DID URL values only,
   opt-in, non-secret, package/runtime tested, observable, and reversible. It
   neither modifies SDK-TS in this change nor promises a default switch.
6. Platform responsibilities remain cohesive. npm exports, plugin host,
   browser/Node networking, storage runtime, lifecycle, and orchestration are
   not moved merely to maximize Rust ownership.
7. The blueprint no longer proposes `identus-apollo` or `identus-did-core` as
   target crates; it names the actual responsibility-owned `identus-crypto`
   and `identus-did` crates while retaining Apollo only as comparison evidence.

No correctness, security, target-support, performance, certification, or
consumer-adoption claim is inferred from documentary validation. Those claims
remain gated by their bounded follow-up issues.
