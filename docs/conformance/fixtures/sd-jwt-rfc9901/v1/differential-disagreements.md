# RFC 9901 differential disagreement ledger

Issue: #568. Scope: the stable RFC 9901 compact base format only.

| Oracle | Immutable input | Normalized comparison | Result | Disposition |
|---|---|---|---|---|
| `affinidi-sd-jwt` 0.1.8 / `95fde42bd338f308ea0ce8473a85a92b2f5f5423` | crates.io archive SHA-256 `b44d072f5e9be444fdf1e4c458175b567ded9f46119ab6c5b2b8cc6ad1ccb1cb` | Exact RFC disclosure bytes and base64url SHA-256 digest | agreement | Retain as an offline oracle; do not add it to the production graph. |
| IOTA `sd-jwt-payload` 0.5.1 / `c47e52c31545f7ede55ad1259f275f31595e2322` | crates.io archive SHA-256 `3d0271aef808decf6005e82e2b779a5659a630855206cde7e89a86d0d2b195cd` | Exact RFC disclosure bytes and base64url SHA-256 digest | agreement | Retain as an offline payload oracle; do not import its envelope or DTO model. |

The executable conformance test compares only stable wire semantics. Foreign
DTOs, error strings, trust policy, randomness, JOSE stacks, and dependency
graphs are deliberately excluded. There are no unresolved disagreements in
this packet. Any future disagreement must be recorded here and resolved against
RFC 9901 before changing SDK behavior.
