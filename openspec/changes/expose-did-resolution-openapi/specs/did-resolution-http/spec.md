# DID Resolution HTTP delta

## ADDED Requirements

### Requirement: Optional OpenAPI document matches the fixed resolver

The adapter SHALL expose an `openapi` feature, disabled by default, that returns
a deterministic OpenAPI 3.1 document for the mount-relative `GET /{did}` route.
The document SHALL describe the required DID path, the common GET resolution
options, method-specific string option extensibility, implemented input bounds,
the three supported success representations, standard error status mapping and
`Vary: Accept` response behavior.

The feature SHALL use a typed OpenAPI model without enabling generator macros,
and SHALL NOT add an OpenAPI dependency or annotation to `identus-did`. It SHALL
NOT describe POST, DID URL dereferencing, a dynamic mount path, a server runtime,
middleware, authorization, deployment policy or any other behavior absent from
the router.

#### Scenario: Feature user obtains a composable document

- **WHEN** a host enables `openapi` and requests the adapter document
- **THEN** it receives one mount-relative GET operation that it can merge into its host document

#### Scenario: Document matches implemented transport behavior

- **WHEN** the document is serialized
- **THEN** its parameters, media types, statuses, bounds and response headers match the fixed router contract

#### Scenario: Unsupported surfaces stay absent

- **WHEN** the document paths and operations are inspected
- **THEN** no POST, dereferencing, dynamic path, listener, middleware or authorization surface is advertised

#### Scenario: Default consumers retain the narrow graph

- **WHEN** the adapter is built without `openapi`
- **THEN** Utoipa and its macro package are absent from that feature graph and runtime behavior is unchanged
