# A1 shared compatibility foundation delivery plan

**Milestone:** [A1 — Shared compatibility foundation](https://github.com/hyperledger-identus/sdk-rust/milestone/5)

**Parent:** [#504](https://github.com/hyperledger-identus/sdk-rust/issues/504)

**Machine plan:**
[`a1-shared-compatibility-foundation.toml`](a1-shared-compatibility-foundation.toml)

## Outcome

A1 delivers one shared evidence language for every later cross-language
capability. It is complete when vector provenance, canonical adapter/error
mappings, consumer-visible changes, and risk-routed quality evidence form one
offline-valid graph seeded by generic DID/DID URL behavior.

A1 does not change SDK-TS or implement peer DID, SD-JWT, AnonCreds, DIDComm, an
agent runtime, networking, or a new binding.

## Work graph

| Order | Issue | Capability | Blocking relation | Closeout evidence |
| ---: | --- | --- | --- | --- |
| 1a | [#420](https://github.com/hyperledger-identus/sdk-rust/issues/420) | source-of-truth vector catalog | none | schema, validator/mutations, DID packet, Rust selector receipt |
| 1b | [#505](https://github.com/hyperledger-identus/sdk-rust/issues/505) | canonical adapter/error mappings | none | schema, validator/mutations, DID value/error mappings |
| 2a | [#501](https://github.com/hyperledger-identus/sdk-rust/issues/501) | risk-routed quality evidence | blocked by #420 for closeout | four-class schema/validator and DID declaration |
| 2b | [#422](https://github.com/hyperledger-identus/sdk-rust/issues/422) | consumer change ledger | blocked by #420 and #505 | schema v2, validator/renderer, cross-reference evidence |
| 3 | [#504](https://github.com/hyperledger-identus/sdk-rust/issues/504) | combined milestone receipt | all four sub-issues | combined offline validation and protected CI receipt |
| next milestone | [#492](https://github.com/hyperledger-identus/sdk-rust/issues/492) | SDK-TS DID canary | blocked by #420/#422/#501/#505 | same packet passes Rust and versioned TS adapter |

Native GitHub sub-issues and blocked-by links encode this graph. The table is a
review aid, not a second issue backlog.

## Agent-ready entry contract

An agent may start a child only when:

1. the issue is open, assigned to milestone 5, and its GitHub blockers are
   closed or the work is explicitly limited to non-conflicting design;
2. the child OpenSpec names one contract and its exact paths;
3. research classifies standards, donor evidence, licenses, dependency/target
   impact, security/privacy, and unrun checks;
4. the branch follows `codex/<type>/issue-<number>` and obtains a planning-only
   preimplementation receipt; and
5. the PR declares exact closure evidence and stop boundary.

Agents do not create more issues unless they discover a separately owned
capability that cannot close inside the four contracts. Implementation details
stay as checkboxes/comments on the owning issue.

## Definition of done

- all four native sub-issues are closed with their own evidence comments;
- each schema and validator has positive and mutation coverage;
- the first generic DID/DID URL packet has immutable provenance and no private
  data;
- every cross-record ID resolves and no payload is duplicated for convenience;
- Rust DTO/error contracts remain canonical and language mappings are bounded;
- all four quality classes are exact or justified not applicable;
- the combined factory check and protected PR CI are green; and
- #492 remains open as the next adoption proof.

## Pull-request and merge plan

The preferred sequence is two parallel roots (#420/#505), then two shallow
dependants (#501/#422), then a small #504 closeout. Do not create a long-lived
milestone branch: the contracts are designed to merge independently into
protected `develop`, and stacked rebases add latency without isolating more
behavior.

Every PR links its issue, contains signed/DCO commits, publishes local factory
metrics to the issue or PR, and may merge when required fast CI and review are
green. Slow target evidence is weekly/release-routed unless a child changes a
target contract.

## Stop boundary

After #504 closes, stop and reassess A2. Do not let completion of planning or
infrastructure silently authorize SDK-TS mutation, peer DID, Prism method,
credential format, DIDComm, runtime, networking, or FFI implementation.
