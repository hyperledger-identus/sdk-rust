# Design

## Protocol-owned values

`CredentialFormat` is an SDK-owned closed-known/open-extension identifier. The
core recognizes Final identifiers while retaining a bounded extension value.
`TransactionData` owns the exact encoded string, decoded type, referenced DCQL
credential IDs, and bounded type data behind redacted accessors. It is created
only by the request transition.

## Capability ports

A caller supplies a finite slice of `CredentialFormatCapability` objects and a
finite slice of `TransactionDataCapability` objects. Format ports validate one
format's bounded DCQL metadata and declare whether a transaction type can bind
to that format. Transaction ports validate type-specific object semantics and
return an SDK-owned binding instruction. Static port error categories are
mapped to stable OID4VP errors; no foreign error escapes.

There is no global registry. Duplicate registrations and missing capabilities
fail before callback execution. Empty metadata uses a built-in baseline rule;
non-empty metadata always requires a port.

## Consumed lineage

Authorization parsing validates DCQL format/meta first, then parses transaction
envelopes, resolves each `credential_ids` reference against the validated DCQL
query, and retains the result beside that query. A later selection-validation
method consumes or borrows DCQL result IDs plus selected credential formats and
returns per-presentation transaction binding inputs. It rejects unavailable or
incompatible referenced credentials before any proof engine runs.

The binding input exposes the exact encoded transaction string only through an
explicit sensitive accessor so SD-JWT hashing and mdoc embedding can occur in
their owning crates without reserialization.

## Evidence architecture

Clean-room E2E follows signed JAR through verifier authorization, DCQL, format
metadata validation, selection, and binding input. Adapter canaries implement
small consumer-owned SD-JWT-shaped and mdoc-shaped capabilities without foreign
types. Negative evidence covers malformed encoding/JSON, unknown type/format,
duplicate registrations, bad references, incompatible selection, limits, and
diagnostic leakage. Property/fuzz/benchmark evidence shares the public parser.
