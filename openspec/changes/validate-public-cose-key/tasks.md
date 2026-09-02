## 1. Contract and architecture

- [x] 1.1 Create child issue #30 under IDR-004 parent #9 with normative,
      source, license, scope, threat, bounds, verification, rollback and
      adoption data
- [x] 1.2 Add the OpenSpec proposal, crypto delta, design and ADR 0006 before
      implementation; record semantic review with no uncleared blocker

## 2. Validated public COSE Key

- [ ] 2.1 Add optional workspace `coset`, the `cose` feature and SDK-owned
      typed public-key representation plus redaction-safe errors
- [ ] 2.2 Implement bounded exact-end parsing, structural/private validation,
      extension retention and deterministic recursive CBOR encoding
- [ ] 2.3 Implement full-coordinate JWK conversions and compressed-point
      rejection without importing algorithm policy
- [ ] 2.4 Implement `EncodeCose` for all four current curve public keys

## 3. Conformance and misuse resistance

- [ ] 3.1 Add assigned/text registry, OKP/EC2, compressed/full, deterministic
      and extension round-trip fixtures
- [ ] 3.2 Add negative tests for every recorded shape, private, duplicate,
      trailing, tag, resource and redaction threat
- [ ] 3.3 Run focused, workspace, no-default, minimal-feature, wasm, MSRV,
      conformance, lint, formatting, docs, supply-chain, OpenSpec and Nix gates
- [ ] 3.4 Record a release-mode encode/parse throughput observation without a
      timing assertion

## 4. Delivery evidence

- [ ] 4.1 Sync the canonical crypto spec, archive the OpenSpec change and
      record exact verification plus a distinct local misuse/security review
- [ ] 4.2 Prepare the issue-linked PR receipt and record exact-head CI,
      independent security review and green-only merge criteria
