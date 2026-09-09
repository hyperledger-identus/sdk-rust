# Why

The OID4VCI roadmap needs authorization-code and PKCE mechanics, but the SDK
must not reimplement closed OAuth algorithms unnecessarily or let a general
OAuth client redefine its bounded protocol types, endpoint policy, secrets,
transport, clock or errors. Issue #160 requests executable evidence before a
production dependency decision.

# What changes

- Evaluate exact `oauth2 5.0.0` with default HTTP clients disabled in a
  separately locked, unpublished consumer fixture.
- Compare deterministic authorization URL and PKCE S256 construction, token
  request formation, response parsing, extension behavior, errors and secret
  diagnostics with the SDK's OID4VCI Final boundaries.
- Record exact version, source, license, MSRV, features, dependency cone,
  unsafe/native reach, targets, security posture and maintenance evidence.
- Decide independently whether authorization URL construction, PKCE mechanics,
  token exchange and response types should be adopted privately, retained
  locally or used only as an oracle.
- Keep the root/runtime dependency graph, public API, wire behavior and fast CI
  unchanged.

# Capabilities

## New capabilities

- `oauth2-authorization-code-spike`: owns the reproducible research contract
  for evaluating external OAuth authorization-code and PKCE mechanics.

# Non-goals

- No production dependency, HTTP client, browser launch, redirect listener,
  runtime, clock, persistence, login UI, trust policy or downstream mutation.
- No public oauth2/url/http/chrono type and no OID4VCI API or wire change.
- No OAuth server, PAR, DPoP, JAR, RAR, refresh, device or client-credentials
  implementation.
- No publication, release or support claim.

# Delivery

Issue #160 owns the spike. Research and material constraints must become ready
before the fixture is implemented. A distinct exact-diff review, complete
evidence receipt, signed/DCO PR to `develop` and green hosted gates are required.
