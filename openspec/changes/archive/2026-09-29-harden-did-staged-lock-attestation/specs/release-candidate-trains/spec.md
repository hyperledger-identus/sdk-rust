# release-candidate-trains

## ADDED Requirements

### Requirement: Candidate lock attestation and refresh are closed

A release-candidate train SHALL attest the digest computed from lock bytes
installed into each staged workspace directly against descriptor-owned source
identity. Every Cargo operation that resolves or inspects staged workspace
sources SHALL use locked resolution. Ordinary candidate and matrix evidence
SHALL reject dependency lock generation after command construction. A distinct
package-extraction closure MAY generate one closure-local lock, and an explicit
review-only refresh mode MAY generate one proposed staged lock outside the
repository. Refresh SHALL run under the pinned primary compiler, report exact
dependency-cone drift, and SHALL NOT edit tracked source identity.

#### Scenario: Four lane receipts agree on a wrong digest

- **WHEN** all submitted lane receipts contain the same validly shaped digest
  that differs from the candidate descriptor
- **THEN** aggregation fails before reporting passing matrix evidence

#### Scenario: Archive receipt is assembled

- **WHEN** staged lock bytes are copied and verified for the archive build
- **THEN** the receipt records the digest returned from those installed bytes
  rather than independently echoing descriptor text

#### Scenario: An alternate helper constructs lock generation

- **WHEN** an ordinary candidate or matrix path constructs an equivalent Cargo
  lock-generation command through a variable or alternate execution helper
- **THEN** runtime command policy and offline structure policy reject it

#### Scenario: A reviewer proposes a lock refresh

- **WHEN** the explicit refresh mode runs under pinned Rust 1.98.1 from a clean
  exact repository revision into a new external output directory
- **THEN** it emits proposed lock bytes and a deterministic closed dependency-
  cone drift report without changing the repository lock or descriptor

#### Scenario: The extracted package closure is verified

- **WHEN** packaged DID crates are patched into the distinct verification
  workspace
- **THEN** that workspace may generate exactly one local lock and all following
  checks and tests consume it with locked resolution
