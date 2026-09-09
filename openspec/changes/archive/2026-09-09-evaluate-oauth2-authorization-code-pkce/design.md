# Context

`identus-oid4vci` currently owns bounded issuer/authorization-server metadata,
authorization-code grant hints, pre-authorized token requests and partial token
responses. It does not yet construct an authorization request or authorization
code token exchange. `oauth2` implements those OAuth mechanics but also owns
URL, HTTP, secret, clock and response models.

# Goals

- Determine whether exact oauth2 mechanics reduce implementation and
  conformance risk behind Identus-owned protocol facades.
- Exercise the candidate as an application would without network access.
- Expose semantic, resource, target and dependency mismatches before adoption.
- Keep research dependencies outside every release graph and ordinary fast CI.

# Decisions

## Use a separately locked consumer fixture

The spike lives outside the root workspace with its own Rust 1.98.1 package and
lock. It depends on exact `oauth2 = 5.0.0` with defaults disabled and on
`identus-oid4vci` only for comparison. The root workspace and package locks
must remain unchanged. The fixture is compiled and tested only by an explicit
research script, not a default feature or PR-fast job.

## Inject deterministic secrets and a recording transport

The fixture supplies fixed CSRF state, authorization code and PKCE verifier so
no candidate RNG becomes SDK policy. A synchronous in-memory HTTP client
captures the token request and returns fixed response bytes; it performs no
network access. Tests inspect generated wire artifacts through public APIs.

## Decide per mechanic

The final ADR classifies authorization URL construction, PKCE S256, token
request encoding and token response parsing independently. A useful closed
algorithm does not imply acceptance of the candidate's endpoint, scope, HTTP,
clock, secret or response types. Any recommended production adoption requires
a separate issue and Identus-owned facade.

## Fail closed on unsupported equivalence

The spike records mismatches rather than normalizing them away. OID4VCI Final
and referenced OAuth RFCs are normative; candidate behavior is evidence only.
No external implementation becomes the protocol authority.

# Compatibility matrix

| Surface | Spike effect |
| --- | --- |
| Root workspace and lock | Unchanged |
| `identus-oid4vci` public/wire API | Unchanged |
| Default/fast CI | Unchanged |
| Research fixture | Exact locked oauth2 dependency; explicit execution only |
| Network, clock and RNG | No network; no clock; fixed caller-owned test secrets |
| WASM/iOS/Android | Compile evidence only where the exact no-default graph permits it |

# Rollback

Remove the fixture, research script, ADR and research capability specification.
No runtime, public API, persisted data, downstream consumer or release artifact
requires migration.
