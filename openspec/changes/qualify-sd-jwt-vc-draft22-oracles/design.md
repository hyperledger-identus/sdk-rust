# Design

## Context

The profile implementation is complete but its differential class is a release
gate. The available independent engines target earlier draft revisions and use
different languages, DTOs, effects, and policy boundaries. Importing them or
comparing raw compact bytes would create coupling without proving semantics.

## Goals / Non-Goals

**Goals:**

- freeze two independent current engines and their complete research receipts;
- compare every required profile surface against the exact draft-22 decision;
- make disagreements and unsupported surfaces durable and reviewable;
- execute a small, fast, offline Rust route on every relevant change;
- keep production and portable dependency graphs unchanged.

**Non-Goals:**

- donor runtime integration, shared DTOs/errors, live cross-language execution
  in normal CI, HTTP, rendering, trust, status acceptance, publication,
  downstream mutation, or automatic legacy compatibility.

## Decisions

### Static packet, reproducible provenance

Candidate runtimes are used only to produce and verify research receipts. The
repository retains a closed JSON packet containing source identities, source
selectors, normalized observations, normative resolutions, and SDK-authored
inputs. Normal CI parses the packet and runs the same semantic cases through
the Rust public API. It never executes donor code.

```text
pinned OWF release ─┐
                    ├─ isolated probe/audit ─► immutable neutral packet
pinned K2GL release ┘                                │
draft-22 hash ───────── normative resolution ───────┤
                                                    ▼
                                      identus-sd-jwt-vc differential test
```

### Closed observation model

Each case has one draft-22 result and exactly one observation per oracle:

- `accept`: the pinned engine accepts or emits the semantic behavior;
- `reject`: the pinned engine rejects it;
- `not-exposed`: the public/tested surface cannot express the decision.

The packet also records an exact upstream source/test selector and a bounded
note code. Free-form donor diagnostics and credentials are prohibited. A
disagreement is expected evidence, not a failing packet, only when its draft
section and normative resolution are present. The Rust result must always
match the normative result.

### Semantic groups

Cases are grouped by protected identity, type claims, protected claim
placement, holder confirmation, Type Metadata, and status references. The
draft-20 mandatory-path and draft-21 non-critical display/`svg_id` changes are
explicit cases because they distinguish draft-22 from the selected releases'
documented draft lineage.

### Supply-chain receipts outside production

The packet manifest records package/archive hashes and license hashes. Separate
bounded receipt files record tool versions, direct and resolved dependencies,
lock hashes, upstream test counts/outcomes, and advisory summaries. Installed
trees and package-manager caches are never checked in. No lifecycle script is
run unless the isolated probe explicitly requires and records it.

### Quality declaration transition

The existing declaration keeps its stable ID. Its differential obligation
changes from `required/pending` with debt #573 to `satisfied/source-bound`,
points at the focused Rust test and packet, records both oracle revisions and
normalization, and clears the debt. Other obligations and the crate's
unpublished limitation remain unchanged.

## Risks / Trade-offs

- Static packets can go stale. Exact versions and source-bound freshness make
  that visible; future refreshes create new packet versions.
- Earlier-draft engines disagree on new semantics. The disagreement ledger
  turns this into useful evidence rather than masking it.
- Cross-language reproduction is slower than Rust CI. It runs during packet
  qualification, while normal CI remains offline and fast.
- Source selectors can move in a mutable branch. Every selector is paired with
  an immutable revision and archive hash.

## Decomposition

This slice should remain under the repository's 12-file/1,000-line guidance
where practical. If immutable receipts require more files, the PR explains the
evidence-shaped decomposition. No production Rust module is added or enlarged.
