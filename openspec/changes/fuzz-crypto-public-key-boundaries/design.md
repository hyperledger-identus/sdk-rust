## Context

`PublicKeyJwk` validates four public profiles, canonical unpadded 32-byte
coordinates, public extensions, serde construction, and RFC 7638 SHA-256
thumbprints. `PublicKeyCose` parses one untagged CBOR map within 4,096 bytes and
16 levels, rejects duplicate/private/float structures, preserves at most 32
additional parameters, emits deterministic CBOR, and converts full-coordinate
key material to and from JWK. Both implementations are safe Rust, but their
untrusted representation surfaces and cross-format invariants benefit from
coverage-guided sanitizer search.

Normative and tooling sources are pinned as follows:

| Source | Immutable pin | Use |
| --- | --- | --- |
| RFC 7517 | <https://www.rfc-editor.org/rfc/rfc7517> | JWK representation |
| RFC 7638 | <https://www.rfc-editor.org/rfc/rfc7638> | JWK thumbprint input and output |
| RFC 8037 | <https://www.rfc-editor.org/rfc/rfc8037> | OKP JOSE profiles |
| RFC 8949 | <https://www.rfc-editor.org/rfc/rfc8949> | deterministic CBOR and duplicate semantics |
| RFC 9052 | <https://www.rfc-editor.org/rfc/rfc9052> | COSE Key structure |
| RFC 9053 | <https://www.rfc-editor.org/rfc/rfc9053> | COSE algorithm/key registrations |
| Rust toolchain | nightly `2026-03-18` | sanitizer-capable compiler |
| Nixpkgs | `7241bcbb4f099a66aafca120d37c65e8dda32717` | reproducible cargo-fuzz `0.13.2` |
| libfuzzer-sys | exact `0.4.13` | runtime binding |

Read-only GitHub evidence was inspected at NeoPRISM `180f1eb`, Apollo
`ccee22b`, midnight-identity `5cb0590`, Lace `217fb78`, and Oxid `c774b6c`.
NeoPRISM has one minimal Rust JWK source; the other repositories provide
compatibility/consumer context. None owns a JWK/COSE sanitizer target. No
donor code or fixture is copied. Full license/status receipts are in issue #58.

## Goals / Non-Goals

**Goals:**

- Search arbitrary hostile JSON and CBOR bytes under AddressSanitizer.
- Exercise rejection and every existing accepted representation path.
- Detect nondeterminism or disagreement across serde, thumbprints,
  deterministic CBOR, and full-coordinate JWK/COSE conversion.
- Cross the public COSE input ceiling while bounding the fuzz harness.
- Preserve fast repeatable PR evidence and separate longer bounded search.
- Keep all fuzz dependencies outside published crates and root lock.

**Non-Goals:**

- New algorithms, curve validation, secret/private-key operations, signatures,
  derivation, JOSE protocols, trust, custody, chain behavior, consumer
  migration, public parser/API limits, OSS-Fuzz, release, or publication.
- Proving absence of bugs or running an unbounded continuous campaign.

## Decisions

### Decision 1: extend the independent workspace without coupling campaigns

The existing `fuzz/` directory remains a standalone workspace and is renamed
internally from a DID-specific package to `identus-sdk-fuzz`. It adds an
`identus-crypto` path dependency with only `jwk-thumbprint` and `cose` features,
plus an exact fuzz-only hex decoder for committed binary seed transport. No
fuzz feature, runner, sanitizer, or dependency enters a published package.

DID and crypto keep separate wrappers and workflows so a source-only crypto
change does not spend DID campaign time and scheduled soak budgets stay under
one workflow's 20-minute ceiling. A shared `fuzz/**` infrastructure change may
exercise both workflows, which is conservative and bounded.

The existing version-exact `libfuzzer-sys@0.4.13` NCSA exception remains
sufficient. Each focused workflow applies formatting, strict Clippy,
cargo-deny, and RustSec to the independent lock before running a campaign.

### Decision 2: JWK fuzzing asserts the validated public contract

The `public_jwk` target gives arbitrary bytes directly to serde JSON. Rejection
is valid. Every accepted JWK must:

- serialize and reparse to an equal value with coherent key type/curve/y shape;
- retain canonical unpadded 32-byte public coordinates and safe extension
  names;
- produce the same 32-byte/43-character unpadded thumbprint before and after
  round trip; and
- convert to COSE and back with identical structural public material.

JWK extensions are intentionally absent after structural COSE conversion; the
target compares required key identity rather than pretending format-specific
metadata survives. The campaign input ceiling bounds fuzz work but does not
become a new public JWK raw-parser promise.

### Decision 3: COSE fuzzing asserts bounded deterministic representation

The `public_cose` target gives arbitrary raw bytes to `from_cbor`. Committed
positive binary seeds use an ASCII `hex:` transport decoded only by the harness
so text-only repository tooling can preserve exact seed bytes; unprefixed
inputs and mutations remain raw CBOR. Invalid hex is clean rejection.

Every accepted key must deterministically encode, reparse equally, retain the
type/curve/coordinate/parameter ceilings, and repeat the same bytes. OKP and
full-coordinate EC2 keys must convert through JWK and back without structural
loss. Compressed EC2 must return its explicit conversion error. One-time raw
4,096/4,097-byte probes exercise the public COSE boundary at startup.

### Decision 4: reuse the proven campaign envelope

The crypto wrapper copies committed corpora into a temporary writable
directory and supplies all libFuzzer controls. Replay uses seed `424242` and
the committed corpus once. PR/push smoke uses one process, seed `424242`, and
4,096 executions per target. Scheduled/manual soak uses at most 300 seconds per
target. All modes cap generated input at 8 KiB, a single execution at five
seconds, RSS at 1 GiB, and disable mutation reload.

Performance is diagnostic only. Generated growth is not committed
automatically. A finding is reproduced, minimized, added to the corpus and a
named deterministic regression when distinct, then fixed or filed. Artifacts
remain untrusted and failure-only.

## Threat Contract

**Assets:** parser availability, public-only enforcement, canonical coordinate
shape, deterministic representation, RFC 7638 identity, structural conversion,
dependency isolation, and downstream compatibility.

**Threats addressed:** malformed/non-UTF-8/deep JSON; private or shadowing
members; invalid coordinates/profiles; malformed/tagged/trailing/duplicate,
deep, floating-point, private, oversized or confusing CBOR; nondeterministic
encoding; conversion metadata invention; compressed-coordinate guessing;
panic, sanitizer failure, and excessive per-input work.

**Residual boundaries:** fuzzing is probabilistic and bounded. It does not
validate curve membership, prove possession, authorize a key, verify a
signature, protect secret material supplied to another API, or establish
product trust. Compiler/platform-specific findings require minimization and a
deterministic regression before compatible production behavior changes.

## Migration and Rollback

No consumer or persisted-data migration exists. Existing DID fuzz commands are
preserved. Crypto contributors use the new wrapper through the locked Nix
shell. Rollback removes the two targets/corpora/dictionaries, wrapper/workflow,
and spec evidence; no published crate, root lock, consumer, chain, or `main`
branch is affected.
