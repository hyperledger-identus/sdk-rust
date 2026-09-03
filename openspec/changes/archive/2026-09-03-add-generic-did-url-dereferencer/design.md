# Design: bounded generic DID URL dereferencing

## Standards and evidence

The implementation follows the current W3C DID Resolution editor's draft at
commit `71a50058090417f9947b9f13985fc8b561a4ad59` (27 August 2026). The generic
algorithm is explicitly at risk, so the public surface is opt-in and removable.
CID error identifiers are pinned at commit
`8d2398eb4d0c1983084e43c762e7f5e3e62b0c1e`.

Normative sources:

- <https://www.w3.org/TR/did-resolution/>, DID URL dereferencing and security.
- <https://github.com/w3c/did-resolution/commit/71a50058090417f9947b9f13985fc8b561a4ad59>.
- <https://github.com/w3c/cid/commit/8d2398eb4d0c1983084e43c762e7f5e3e62b0c1e>.
- RFC 3986 sections 4.2 and 5 for relative references and resolution.

Read-only donor evidence at fixed revisions:

| Repository | Revision | Relevant evidence |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | PRISM document services and relationships; no generic dereferencer |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Midnight DID documents and service metadata; no generic dereferencer |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | wallet consumption shapes; no reusable algorithm |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | relationship/service endpoint consumption; no reusable algorithm |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | legacy identity evidence only; candidate for deprecation |

No implementation or fixture is copied.

## Composition boundary

`GenericDidUrlDereferencer` owns an `Arc<dyn DidResolver>` and implements the
existing object-safe `DidUrlDereferencer` port. It is a deterministic domain
orchestrator, not a registry and not a transport. Existing method-specific
dereferencers stay independently selectable through `DidMethodRegistry` or an
outer strategy chain.

Unrecognized custom path/query resource semantics return standard `notFound`.
This prevents a generic fallback from guessing method or extension behavior.
The injected resolver receives the base DID and projected resolution options
exactly once per call.

## Parameter preparation

The raw query is split on `&`, each name/value is percent-decoded exactly once,
and `+` remains a literal plus. Empty names, malformed escapes, control bytes,
invalid UTF-8, bounds violations and duplicate decoded names produce
`invalidDidUrl`. Duplicate rejection makes scalar standard parameters
unambiguous and prevents normalization bypass.

`versionId` and `versionTime` populate their typed `ResolutionOptions` fields;
`hl` and other parameters are retained as bounded extensions. The dereference
`accept`, `verificationRelationship` and extension options are also projected
without permitting collisions with URL parameters. Known resource selectors
`service`, `serviceType` and `relativeRef` are consumed by the generic layer.

A URL containing only resolution parameters still identifies the DID document.
A non-empty path or unknown resource query is not interpreted generically.

## Document and fragment processing

After successful resolution, a missing document becomes `notFound`; upstream
resolution errors retain their W3C URI. A bare DID returns the full document
and its document metadata.

A fragment is matched by exact absolute resource identifier based on the
resolved document DID, never by prefix or decoded string equivalence. The
algorithm searches top-level and relationship-embedded verification methods,
then services. Unknown fragments return `notFound`.

When `verificationRelationship` is supplied, the fragment must identify a
verification method and the requested relationship must be one of the five DID
Core relationships. Missing methods use CID `INVALID_VERIFICATION_METHOD`;
unknown or unassociated relationships use CID
`INVALID_RELATIONSHIP_FOR_VERIFICATION_METHOD`. Membership accepts either an
exact reference or an embedded method with the exact identifier.

## Service selection and representations

`service` accepts an absolute service identifier, `#fragment`, or a bare local
fragment name; local forms are expanded against the resolved document DID.
`serviceType` matches an exact declared service type. When both are present,
selection is conjunctive. No match returns `notFound`.

Absent `accept` and DID document media types return a cloned document whose
service property contains only selected services. `text/uri-list` returns a
bounded JSON array of string endpoint URIs and records that media type in
dereferencing-operation metadata. Endpoint maps and empty endpoint sets
contribute no URI.
Other requested media types return `representationNotSupported`.

`relativeRef` requires service selection and operates only on string endpoint
URIs. If a dereferenced input fragment is to be inherited, exactly one endpoint
must remain and it must not already contain a fragment.

## Safe relative-reference policy

RFC 3986 resolution is necessary but insufficient for wallet-grade service
routing. The generic layer therefore applies an intentionally conservative
policy:

1. the decoded value must be a relative reference with no scheme, authority or
   backslash;
2. repeated validation-only percent decoding must stabilize within a bounded
   number of passes and must not reveal control bytes, backslashes or `.`/`..`
   path segments;
3. RFC 3986 merging and dot-segment removal produce the candidate URI;
4. candidate scheme/authority must equal the endpoint and its normalized path
   must stay within the endpoint's base directory scope;
5. all intermediate and final values remain under existing 4 KiB/64 KiB
   structural bounds.

Query-only and fragment-only relative references are allowed because they do
not change path scope. Encoded and double-encoded traversal, network-path
references and absolute references fail as `invalidOptions`. This policy is
stricter than generic browser navigation by design and is independently
removable if the at-risk W3C feature changes.

## Error, metadata and observability contract

Invalid URL syntax/parameters use `invalidDidUrl`; contradictory or unsafe
caller controls use `invalidOptions`; unsupported output uses
`representationNotSupported`; absent resources use `notFound`. Resolver errors
pass through. CID relationship errors use their exact open URI identifiers.

Errors and `Debug` output never include full DIDs, selectors, endpoints,
relative references, documents or adapter errors. Successful document/resource
results preserve bounded document metadata. URI-list results add only the
selected media type.

## Concurrency and performance

The adapter is immutable, `Send + Sync`, cloneable through `Arc`, and performs
no shared mutation. Concurrent calls inherit the resolver's guarantees. Tests
cover two representative PRISM/Midnight mock resolvers, malformed/adversarial
inputs and object-safe concurrent calls. A release-mode diagnostic records a
representative bare-document throughput without a machine-specific CI limit.

## Alternatives rejected

- **Put dereferencing in each DID method:** duplicates generic W3C semantics and
  makes relationship/service security inconsistent.
- **Fetch selected endpoint URLs:** couples the DID domain to transports,
  redirects, SSRF policy and runtime behavior.
- **Use form-url-encoding:** changes literal plus signs and can alias selectors.
- **Permit duplicate scalar parameters:** leaves ordering/override semantics
  exploitable and underspecified.
- **Return every endpoint JSON shape as a URI:** invents method/product meaning
  for endpoint maps.
- **Allow ordinary `../` resolution:** permits a DID document to escape the
  advertised endpoint route and creates encoded traversal ambiguity.

## Rollback

Revert issue #46's pull request. The adapter is opt-in, unpublished, stores no
state, performs no external retrieval and does not change existing registry or
resolver behavior.
