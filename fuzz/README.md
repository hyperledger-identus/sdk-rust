# SDK boundary fuzzing

The standalone fuzz workspace searches public DID lexical and cryptographic
representation boundaries without adding fuzz dependencies to SDK crates.

Enter the repository's Nix shell or let the wrapper enter it for you:

```console
./scripts/fuzz-did.sh replay all
./scripts/fuzz-did.sh smoke all
./scripts/fuzz-did.sh soak did_url
./scripts/fuzz-crypto.sh replay all
./scripts/fuzz-crypto.sh smoke public_jwk
./scripts/fuzz-crypto.sh soak public_cose
```

`replay` executes the committed corpus once. `smoke` uses one process, seed
`424242`, 4,096 runs per target, and is the PR/push gate. `soak` uses a fresh
libFuzzer seed and at most 300 seconds per target. All modes cap inputs at 8
KiB, individual executions at five seconds, and RSS at 1 GiB. Targets with
explicit parser bounds also run deterministic exact and one-byte-over probes.

The crypto lane covers validated public JWK parsing, RFC 7638 thumbprints,
bounded public COSE Key parsing, deterministic serialization, and structural
JWK/COSE conversion. Committed COSE examples use the reviewable `hex:` text
transport; arbitrary unprefixed fuzzer input is interpreted as raw CBOR.

Generated campaign growth runs in a temporary copy, so it cannot dirty the
curated corpus. Findings are written beneath `fuzz/artifacts/` and must be
treated as untrusted, potentially sensitive data. For a real finding:

1. reproduce it with `cargo fuzz run <target> fuzz/artifacts/<target>/<file>`;
2. minimize it with `cargo fuzz tmin <target> <artifact>`;
3. copy the minimized case into `fuzz/corpus/<target>/`;
4. add a named deterministic crate test when it captures a distinct defect;
5. fix or file the defect before discarding the artifact.

Bounded fuzzing is evidence, not proof of correctness. DID method semantics,
resolution, chain state, cryptographic arithmetic, trust and wallet policy
remain outside these targets.
