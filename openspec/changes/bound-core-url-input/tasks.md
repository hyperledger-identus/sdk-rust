# Tasks

- [x] 1.1 Create issue #193 as a focused child of #168.
- [x] 1.2 Inspect the current constructor funnel, consumer surface, existing
      SDK byte limits and authoritative RFC guidance.
- [x] 1.3 Record material constraint, compatibility, activation and rollback
      decisions before implementation.
- [x] 2.1 Add and export `MAX_URL_BYTES = 8_192`.
- [x] 2.2 Add `UrlError::TooLong`, perform the byte check before syntax work,
      and preserve the stable redaction-safe core mapping.
- [x] 2.3 Add exact-boundary, all-constructor, serde and multibyte tests.
- [x] 2.4 Update canonical spec, public docs and `SDK-LIM-007` atomically.
- [ ] 3.1 Run focused tests, formatting, Clippy, feature/dependency checks and
      complete Nix validation.
- [ ] 3.2 Perform distinct exact-diff/security review and resolve findings.
- [ ] 3.3 Record verification, archive the OpenSpec change, and validate the
      factory archive contract.
- [ ] 4.1 Open a signed, DCO-compliant PR to `develop` linked to #193.
- [ ] 4.2 Merge only after every hosted gate is green; leave #168 open for the
      remaining inherited-bound inventory.
