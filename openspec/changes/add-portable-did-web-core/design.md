# Design

## Architecture

```text
DidWeb::parse
      |
      v
pure HTTPS request derivation
      |
      v
DidWebFetcher port  <--- runtime adapter owns network and QoS policy
      |
      v
bounded response binding -> identus-did parser -> exact document ID
```

`identus-did-web` owns method syntax, deterministic URL derivation, response
envelope validation, DID document parsing, and correlation. It does not own a
network runtime. The public fetch trait follows the repository's boxed-future
port convention and remains implementable without an async-trait macro.

## Public model

- `DidWeb`: validated DID plus its derived request URL.
- `DidWebLimits`: strictly positive public identifier, segment, segment-count,
  URL, media-type, and response-body ceilings with conservative defaults.
- `DidWebRequest`: immutable GET request data exposing URL and accepted media
  types; no arbitrary headers, body, credentials, redirect target, or method.
- `DidWebResponse`: status, optional media type, and owned bytes constructed
  under the request's response limits.
- `DidWebFetcher`: caller-implemented asynchronous port returning a response or
  the content-free `TransportFailure` category.
- `DidWebResolver`: composes a validated method value and fetch port into the
  generic `DidResolver` contract.

The core also exposes synchronous `bind_response` behavior so adapters can
separate I/O from protocol validation and test it without an executor.

## Validation pipeline

Parsing checks total length and `did:web:` before generic DID construction. It
then splits the method-specific identifier into authority and path. Authority
validation recognizes at most one uppercase `%3A` port delimiter and rejects
every other percent sign. FQDN labels are lowercase ASCII, 1..=63 bytes, do not
start/end with a hyphen, and total at most 253 bytes. Ports use canonical
decimal syntax and exclude 443. Paths have finite count/length, use unreserved
ASCII, and reject `.` and `..`.

URL derivation is a direct allocation after checked length arithmetic. Bare
domains receive `/.well-known/did.json`; path DIDs receive their segments and
`/did.json`. No URL parser is allowed to reinterpret the validated authority.

## Retrieval and binding

Resolution creates one request and invokes the injected port once. A transport
failure maps to a fixed method error. Binding requires status 200, a bounded
supported JSON media type (parameters are accepted only after syntactic
validation), and a non-empty body no larger than the configured ceiling. The
existing bounded DID document parser rejects duplicate names, excessive JSON,
and invalid document structure. Exact ID mismatch is a distinct content-free
failure.

Only HTTP 404 maps to the standard `notFound` result; other non-success
statuses remain internal/transient so the generic cache cannot retain an
outage as a missing DID. Resolver representation metadata is derived from the
requested representation only after the response agrees with it. JSON-LD
requires both `application/did+ld+json` and a document context.

Generic `DidResolver` integration returns standard DID resolution metadata and
errors. The method-specific synchronous API retains more precise fixed error
categories for direct adopters.

## Documentation and E2E

The handbook explains ownership, security responsibilities, strict profile,
adapter recipe, and limitations. Architecture diagrams distinguish pure core
from runtime policy. The in-memory E2E implements the public port, records the
request, returns owned response bytes, and runs the same resolver path as a real
adapter. It covers bare domain, path, encoded port, invalid response, oversized
response, and wrong-document ID without network access or timing flakiness.

## Rollback and extension

The crate is additive and opt-in. A future concrete adapter, IDNA profile,
alternate representation, or relaxed domain profile requires a separate ADR
and capability issue; none can widen this core implicitly.
