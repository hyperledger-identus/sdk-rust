# OAuth2 authorization-code spike delta

## ADDED Requirements

### Requirement: OAuth mechanics are evaluated outside release graphs

The SDK SHALL evaluate exact `oauth2 5.0.0` with default features disabled in a
separately locked unpublished fixture. The candidate and every transitive type
SHALL remain absent from root runtime, feature, lock and public API surfaces.
The fixture SHALL execute only through an explicit research command and SHALL
perform no network request.

#### Scenario: Ordinary consumers remain unchanged

- **WHEN** the root workspace is built or its release dependency graph is inspected
- **THEN** oauth2 and its research-only dependencies SHALL be absent

### Requirement: The spike is consumer-shaped and deterministic

The fixture SHALL use caller-supplied CSRF state, authorization code and PKCE
verifier, construct an authorization URL and token request through public
candidate APIs, and capture the request through an in-memory injected client.
It SHALL compare generated values with normative expected bytes and SHALL NOT
select a clock, RNG, browser, redirect listener, HTTP stack or runtime.

#### Scenario: Authorization code with PKCE is exercised

- **WHEN** the deterministic consumer flow runs
- **THEN** the S256 challenge, authorization request and token request match the pinned RFC/OID4VC profile expectations

### Requirement: Reuse decisions are per mechanic

The final research and ADR SHALL classify authorization URL construction, PKCE
S256, token request construction and token response parsing independently. It
SHALL record exact source/license/MSRV/features/cones, unsafe/native reach,
target results, security/resource/secret behavior, protocol currency,
maintenance, compatibility, rollback and objective reconsideration triggers.

#### Scenario: A useful subset does not admit the whole client

- **WHEN** one mechanic passes while another conflicts with SDK policy
- **THEN** the passing mechanic MAY be recommended only behind a future Identus-owned facade while the conflicting mechanic is rejected or retained as an oracle

### Requirement: Research success does not activate production use

The fixture SHALL NOT modify `identus-oid4vci`, downstream repositories,
support policy or release artifacts. A production recommendation SHALL require
a separate issue and spec-driven delivery with bounded Identus-owned inputs,
states, errors and transport ports.

#### Scenario: Spike merges

- **WHEN** the research evidence merges to `develop`
- **THEN** current OID4VCI APIs and wire behavior remain unchanged and no oauth2 production dependency or support claim exists
