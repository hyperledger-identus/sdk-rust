# Local review

Codex CLI reviewed the complete change against `origin/develop` after Claude
CLI was unavailable. The required fallback prevented an external session limit
from blocking delivery.

## Resolved findings

- Malformed non-ASCII numalgo-2 purpose elements no longer panic.
- Numalgo-4 rejects secret-shaped and duplicate JSON members before retention;
  its `Debug` output is redacted.
- Both profiles guarantee that accepted created or parsed DIDs resolve locally;
  service URIs and post-contextualization alias bounds are validated.
- Numalgo-4 parsing enforces the decoded byte ceiling without a conservative
  base58 estimate that could reject values produced by creation.
- The fuzz target treats bounded construction rejection as an ordinary outcome.
- The exact 16 KiB decoded input ceiling round-trips. The profile ceiling is
  intentionally below the generic DID limit because large base58 conversion is
  super-linear; the measured exact-boundary test completes in about 2.3 seconds
  in the debug profile on the local Apple Silicon host.

## Rejected finding

The review suggested fingerprint-derived numalgo-2 verification method IDs.
The pinned DIF specification at
`decentralized-identity/peer-did-method-spec@c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8`
requires ordered `#key-N` identifiers starting at one, and the pinned
`did-peer` 0.7.5 oracle implements the same rule. The design text was corrected
to state ordered-position derivation; the implementation remains unchanged.
