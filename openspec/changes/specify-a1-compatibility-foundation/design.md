# Design

## Four cohesive records

The A1 foundation uses four independently versioned machine records:

1. The vector catalog owns source provenance, authority class, immutable
   payload identity, normative/profile scope, expected result, redistribution,
   capability owner, and exact test selectors.
2. The adapter map owns a canonical Rust DTO/error reference and language-
   specific legacy/additive mappings, compatibility direction, version window,
   deprecation/rollback, and vectors that prove the mapping.
3. The change ledger owns consumer-visible classification, old/new behavior,
   affected packages/capabilities/mappings, migration action/window, release-
   note class, rollback, and exact evidence.
4. The quality declaration owns property, fuzz, benchmark, and differential
   obligations, exact selectors/artifacts, thresholds or seeds, target/risk
   routing, and a reviewed not-applicable rationale where necessary.

Each file has one schema version and stable IDs. References are acyclic: vector
and adapter IDs exist first; ledger and quality records may reference them.
Payloads are never copied merely to satisfy another schema.

## Delivery graph

```text
                    #504 A1 parent
                    /            \
       #420 vector authority    #505 adapter mappings
                    \            /
                     \          /
                      #422 change ledger

       #501 quality routing may design independently,
       then binds its first declaration to #420's DID packet.

       #420 + #505 + #422 + #501
                       |
                       v
              #492 SDK-TS DID canary
```

The parent issue is a milestone receipt, not an extra implementation layer.
Checklists summarize closure while native GitHub sub-issue and dependency
relationships remain the authoritative work graph.

## First DID packet

The first packet uses bounded DID and DID URL values already supported by
`identus-did`. It includes W3C-shaped positive cases, grammar negatives,
redacted error expectations, byte-limit boundaries, and selected copied
TS/Swift/KMP examples classified as `consumer-regression`. It excludes peer
DID resolution/creation, Prism operation encoding, chain behavior, and SDK-TS
adapter execution.

The same immutable packet will later be loaded by a Rust harness and the
downstream TypeScript canary. Language-specific assertions are expressed by
adapter mappings, not embedded in the shared payload.

## Validation and evolution

Offline validators reject unknown fields, duplicate IDs, dangling references,
invalid hashes or revisions, absent authority/licensing decisions, invalid
dependency order, and quality declarations without exact outcomes. Mutation
tests remove or corrupt every mandatory relationship.

Schema evolution is additive only within a schema version. A semantic change
increments the version and requires migration evidence. Source retrieval is a
research/update operation; ordinary CI never depends on donor availability.

## Completion and non-claims

Each child closes with its schema, validator, mutation tests, seed records,
documentation, and evidence comment. #504 closes only when all four children
are complete and the combined offline graph validates. That closes the A1
foundation; it does not close #492 or claim any language SDK consumes Rust.
