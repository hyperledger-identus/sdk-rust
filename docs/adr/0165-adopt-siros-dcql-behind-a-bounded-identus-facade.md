# ADR 0165: adopt SIROS DCQL behind a bounded Identus facade

- **Status:** Accepted under the IDR-024 roadmap mandate
- **Date:** 2026-09-29
- **Issue:** [#429](https://github.com/hyperledger-identus/sdk-rust/issues/429)
- **Supersedes in part:** ADR 0156 production-adoption deferral

## Context

The OID4VP crate now has a named signature-verified payload consumer. ADR 0156
showed that exact `siros-dcql 0.3.0` provides cohesive Final-profile path and
selection logic with a small portable cone, while its public parser, models,
collections, and diagnostics do not meet SDK bounds or redaction policy.

## Decision

Adopt exact `siros-dcql 0.3.0` as a private implementation dependency of
unpublished `identus-oid4vp`. An Identus-owned facade validates stricter Final
structure, references, resource/work limits, and known tolerance mismatches
before candidate parsing. Only SDK-owned credential/path/result/error types are
public. The first policy supports exact format matching and rejects non-empty
format metadata or trusted-authority constraints.

`VerifiedRequestObject` is consumed into `ValidatedDcqlQuery`; this proves only
signature provenance plus DCQL structure. It does not prove full Authorization
Request validity, verifier trust, credential validity, consent, or response
safety.

## Consequences

The SDK avoids maintaining the candidate's generic path, holder-binding,
claim-selection, credential-set, and combination engine while retaining its
own protocol/security boundary. The dependency remains replaceable because no
candidate type escapes. Format-specific policy is deferred rather than
silently approximated.

## Rejection gate

Reverse this decision before merge if the strict adapter duplicates most of
the candidate's selection engine, if the incremental cone or target evidence
regresses, or if bounded work cannot be proven before candidate allocation.

## Verification and rollback

Clean-room differential and adversarial vectors, exact cone/source evidence,
primary/MSRV/portable compiles, public-type isolation, strict Clippy/docs, and
exact-diff review are required. Rollback removes the additive unpublished API
and private dependency without migration.
