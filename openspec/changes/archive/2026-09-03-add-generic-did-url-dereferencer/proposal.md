# Change: add bounded generic DID URL dereferencing

## Why

The SDK has validated DID URLs, bounded DID documents, current W3C resolution
envelopes and object-safe resolver/dereferencer ports. It does not yet provide
the reusable algorithm that turns a DID URL into a document resource or
service endpoint. Every method and wallet would otherwise reproduce parameter
parsing, fragment matching, verification-relationship checks and service
selection with subtly different security behavior.

The current W3C DID Resolution editor's draft marks its dereferencing algorithm
at risk and permits DID methods, extensions and clients to handle custom paths
and query parameters. This slice therefore implements only the portable,
testable intersection: resolution-option projection, bare documents, exact
document resources, relationship authorization and safe service endpoint URI
selection. Network retrieval and method-specific behavior remain outer.

## What changes

- Add an opt-in `GenericDidUrlDereferencer` over any injected `DidResolver`.
- Parse bounded DID URL parameters without HTML form semantics, reject malformed
  percent encoding and duplicate names, and project resolution parameters.
- Return bare documents and exact verification-method or service fragments.
- Enforce the five DID Core verification relationships when requested.
- Select services by `service` and/or `serviceType`, returning either a filtered
  DID document or a bounded `text/uri-list` value.
- Resolve a single `relativeRef` against string service endpoints with an
  explicit path-scope policy resistant to encoded traversal.
- Preserve W3C/CID error identity, document metadata and redaction-safe bounds.
- Record deterministic conformance, misuse, concurrency and release performance
  evidence.

## Boundaries

- The algorithm performs no HTTP, redirect following, DNS, VDR, chain or file
  access beyond the injected resolver call.
- Method-specific, extension and client retrieval strategies remain independent
  `DidUrlDereferencer` implementations selected by outer composition.
- Service endpoint maps are not interpreted as URLs and are skipped for
  `text/uri-list` projection.
- Recursive retrieval, cycle/depth controls, SSRF policy and transports remain
  tracked by #10; cancellation-safe resolver single-flight remains #50.
- No downstream repository is modified and no donor source is copied.

## Delivery

Issue #46 precedes this specification and implementation. Delivery uses ADR
0014, signed+DCO commits, distinct local semantic/security/API review, complete
Nix gates and exact-head hosted CI before merge to `develop`.
