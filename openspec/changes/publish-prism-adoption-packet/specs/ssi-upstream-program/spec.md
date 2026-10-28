## ADDED Requirements

### Requirement: Mixed DID-method portfolio delivery is explicit

The IDR-040 backlog row SHALL distinguish delivered peer/PRISM method profiles
from DID methods that still require their own focused specification and SHALL
NOT mark the complete portfolio delivered while those methods remain absent.

#### Scenario: PRISM and peer slices are delivered

- **WHEN** issues #521, #522, and #523 have immutable merged evidence but `did:key`, `did:jwk`, or `did:web` remain unplanned
- **THEN** IDR-040 is reported as `in_progress` under a focused closeout owner with the delivered and remaining profiles named explicitly
