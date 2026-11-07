# TS1 — SDK-Rust/SDK-TS capability parity and conformance

**Status:** baseline defined; no SDK-TS route is claimed delivered

**GitHub milestone:**
[TS1 — SDK-Rust/SDK-TS capability parity and conformance](https://github.com/hyperledger-identus/sdk-rust/milestone/7)

**Machine record:**
[`sdk-ts-parity-conformance.toml`](sdk-ts-parity-conformance.toml)

**Source baseline:** `@hyperledger/identus-sdk@8.1.4` at
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`

## Outcome

TS1 makes cross-language progress reviewable without pretending that Rust and
TypeScript should have identical repositories, modules, APIs, or runtime
responsibilities. It classifies every normalized SDK-TS capability twice:

1. a **relation** says what parity should mean for that capability; and
2. a **state** says what the current immutable evidence actually proves.

The counts below are an inventory check, not a parity score. There is no single
repository-wide percentage because `platform-owned`, intentionally different,
blocked, and deferred work have different meanings.

## Relation vocabulary

| Relation | Meaning |
| --- | --- |
| `semantic-equivalent` | The selected profile must produce the same bounded domain or protocol result in both SDKs. |
| `compatible-adapter` | Rust owns portable semantics; TypeScript may keep language DTOs, errors, async conventions, custody, networking, lifecycle, or packaging behind a versioned adapter. |
| `platform-owned` | The responsibility is inherently browser, Node, npm, plugin-host, UI, permission, or product lifecycle behavior. Rust exposes only the necessary port. |
| `intentionally-different` | Rust follows an accepted newer or safer profile; the migration and compatibility behavior are explicit. |
| `deferred` | Product, profile, dependency, or consumer evidence is not sufficient to select a parity target. |
| `retired` | A donor capability has no continuing target and needs an explicit removal path. |

Delivery state is independent: `delivered`, `partial`, `blocked`, `excluded`,
or `not-applicable`. A mature Rust implementation remains `partial` until the
SDK-TS adapter, consumer, observability, and rollback evidence is complete.

## Current baseline

| State | Count | Capability IDs |
| --- | ---: | --- |
| `partial` | 14 | `crypto.key-operations`, `crypto.hd-derivation`, `did.syntax`, `did.document`, `did.method.peer`, `did.method.prism`, `did.resolution`, `credentials.jwt-vc`, `credentials.sd-jwt`, `credentials.status`, `openid4vc.issuance-wallet`, `wallet.storage-contracts`, `platform.browser-node-networking`, `bindings.wasm-did` |
| `blocked` | 7 | `credentials.anoncreds-v1`, `presentations.exchange`, `messaging.didcomm-v2`, `messaging.didcomm-application-protocols`, `wallet.backup-format`, `agent.protocol-state`, `agent.runtime-orchestration` |
| `not-applicable` | 3 | `wallet.storage-browser`, `platform.typescript-plugin-host`, `platform.npm-packaging` |
| `delivered` | 0 | none |
| `excluded` | 0 | none |

The first activation remains issue #492: an opt-in DID and DID URL facade over
the existing WASM value surface. It proves the migration mechanism and rollback
boundary; it does not imply parity for DID documents, methods, credentials,
messaging, storage, or agent orchestration.

## Local-first capability trains

TS1 implementation should normally be delivered as one cohesive vertical train
per capability or tightly coupled capability cluster. The planning target is
**5,000–10,000 meaningful changed lines** carrying:

- the canonical Rust implementation and applicable public/binding adapter;
- normative, Identus-contract, compatibility, and negative vectors;
- migration, observability, fallback, and rollback behavior;
- documentation and generated-surface updates; and
- focused, mutation, differential, target, and integration tests.

This is not a merge gate or a line-count objective. Generated sources, vendored
code, lockfiles, and evidence artifacts do not count. Unrelated work must not
be bundled to reach the range. A train is split when work has independent
ownership, security or publication authority, rollback, or cannot be reviewed
safely as one unit. A naturally bounded change may be smaller; a larger train
needs an explicit cohesion and reviewability justification.

Signed internal commits are review checkpoints. Targeted local checks run while
the train is assembled; the complete applicable local gate qualifies the
candidate before push. Protected hosted CI remains mandatory final confirmation
and is never weakened or bypassed, but it is not the primary edit/test loop.

## Exit rule

TS1 exits only when each of the 24 records is one of:

- `delivered`, with every required conformance dimension, named consumer,
  observability, fallback, and rollback proved;
- `excluded` or `not-applicable`, with accepted ownership and non-duplication
  evidence; or
- moved to an explicitly accepted later milestone with a concrete blocker and
  no misleading parity claim.

Every delivered capability identifies its selected standard/profile, exact
Rust and SDK-TS revisions, shared vectors, DTO/error mapping, deviations,
resource and security bounds, selected targets, named consumer, and rollback.

## Explicit non-claims

TS1 does not authorize mdoc work, SDK-TS mutation by this repository, a default
binding switch, package publication, stable API status, or service/SDK
deprecation. It does not require historical Apollo, Castor, Pollux, Mercury, or
Pluto module shapes in Rust. Browser storage, plugin hosting, and npm packaging
remain TypeScript responsibilities unless a later reviewed decision changes
their ownership.
