# Verification evidence

- **Issue:** #30 (child of #9 / `IDR-004`)
- **Develop base:** `82819ac622cf601cc1f5c9cb40a71754c53992c0`
- **Reviewed implementation head:** `e166adfdc39afc48648663fcef27357781cc7cf1`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Executed gates

| Command | Result |
| --- | --- |
| `cargo test -p identus-crypto --all-features` | passed; 81 tests, one manual performance test ignored |
| `cargo test -p identus-crypto --test cose --all-features` | passed; 19 contract/security tests, one manual test ignored |
| `cargo build -p identus-crypto --no-default-features --features cose` | passed |
| `cargo build -p identus-crypto --no-default-features --features ed25519,cose` | passed |
| `cargo build -p identus-crypto --target wasm32-unknown-unknown` | passed |
| `cargo test --workspace --all-features` | passed |
| `cargo test --workspace --no-default-features` | passed |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | passed |
| `RUSTDOCFLAGS='-Dwarnings' cargo doc --workspace --no-deps` | passed |
| `cargo test -p identus-conformance` | passed; 20/20 |
| `cargo fmt --all -- --check` and `git diff --check` | passed |
| `./scripts/factory check` | passed; 15/15 strict OpenSpec capabilities and all governance contracts |
| `nix flake check --print-build-logs` | passed every applicable local Darwin flake check after both hosted-review fixes |

The Nix matrix included Rust 1.85 MSRV, default and minimal builds, WASM,
Android, iOS, nextest, clippy, rustdoc, formatting, text/TOML/Nix lint,
`cargo-deny`, RustSec audit and the factory contract. Linux is an incompatible
omitted system on the local Darwin run and is exercised independently by CI.

## Performance observation

Release-mode `PublicKeyCose` encode plus parse completed 50,000 iterations in
77.550709 ms, approximately 644,739 operations per second on this host after
both hosted-review fixes. This is an observation, not a portable timing
threshold.

## Conformance and threat evidence

- Assigned and registered-text OKP/EC2 fixtures cover Ed25519, X25519, P-256
  and secp256k1, including full and compressed EC2 `y` forms.
- Deterministic encoding proves RFC 8949 length-first top-level and nested map
  order, repeatability, retained public extension equivalence, and preservation
  of explicitly present empty `kid` and Base IV byte strings.
- Negative fixtures cover private label `-4`, incompatible/missing shapes,
  coordinate types and widths, duplicate maps, tags, trailing data, input
  size, nesting, the shared common-plus-unknown parameter count, floats and
  compressed-to-JWK conversion.
- Stable `crypto.invalid_cose_key` bridging and debug/display assertions prove
  caller-controlled values do not enter rendered errors.
- All four curve encoders and full-coordinate JWK conversions preserve exact
  public key bytes.

## Provenance recheck

No downstream or donor repository was modified. Their pre-existing state was
unchanged after implementation: Apollo retains its modified nested secp256k1
checkout; midnight-identity retains its untracked/dirty nested midnight-did;
Lace retains `.pi-subagents/`, `.pi/` and `tmp/`; Oxid retains `.claude/` and
`.pi/taskflows/`; NeoPRISM remains clean. Source revisions and license
decisions are recorded in the design and issue.

Issue creation, spec commit and implementation commit occurred at
`2026-09-02T23:12:30Z`, `2026-09-02T23:18:18Z`, and
`2026-09-02T23:33:01Z`, respectively. Hosted CI, security review, merge and
final integration SHAs are recorded on the pull request and issue because they
occur after this immutable local receipt.
