## Context

`identus-did::Multihash` is an existing unvalidated bytes newtype created as a
derive-macro exercise. Its module documentation incorrectly describes the value
as the bytes underlying `did:key`. The did:key method instead specifies
`MULTIBASE(base, MULTICODEC(public-key-type, raw-public-key-bytes))`.
Multihash has a distinct type-length-value structure containing a hash function
code, digest length and digest bytes.

`multihash 0.19.5` remains a good narrow implementation candidate: it is a
bare structural codec, is `no_std` compatible, does not select or implement
hash algorithms, and has a small dependency cone. Technical fit does not by
itself create a product requirement.

## Goals and non-goals

Goals:

- prevent speculative dependencies and premature public semantics;
- correct the did:key/multihash category error in living documentation;
- preserve exact evidence so a future method integration can activate quickly;
- make the consumer-payoff rule reusable by later AI agents.

Non-goals:

- add or exercise the `multihash` crate;
- change, validate, deprecate or remove the current public `Multihash` value;
- define a hash allow-list, multicodec registry, multibase engine or DID method;
- claim that the current hex serde representation is a standards wire format.

## Decisions

### Keep the candidate conditional

Record `multihash 0.19.5` as `conditional-adopt`. A focused DID-method issue may
activate it only when a pinned normative profile actually carries multihash
values and defines allowed codes, digest sizes, maximum capacity, canonical
varints, text/wire encoding and migration behavior.

### Retain the existing placeholder without expanding its meaning

Removing a public value in this decision-only correction would add unrelated
API risk. Keep its exact constructor, accessors, hex formatting and serde
behavior, but describe it neutrally as opaque bytes with no structural or
method guarantee. A future consumer must decide whether to validate/migrate the
type or replace/deprecate it; it cannot silently reinterpret existing hex.

### Gate adoption on consumer payoff

A candidate that passes license, maintenance, security, target, compiler and
dependency-cone checks still remains conditional when no current SDK capability
uses its semantics. Activation must name the consumer and the concrete local
implementation or correctness risk the dependency replaces.

## Risks and mitigations

- **Risk:** retaining a type named `Multihash` may still invite assumptions.
  **Mitigation:** documentation explicitly denies structural and wire-format
  guarantees and links adoption to a separate focused issue.
- **Risk:** deferral causes future duplicate research. **Mitigation:** preserve
  exact version, checksum, release commit, license and measured cone.
- **Risk:** an agent treats a method's multicodec prefix as multihash again.
  **Mitigation:** pin both normative structures in the ADR and source docs.

## Rollback

Reverting this documentation-only change restores the former decision record;
there is no binary, data or API migration. Production adoption still requires
a new issue and change, so rollback cannot remove a runtime dependency.
