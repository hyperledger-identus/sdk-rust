# Reproducible SD-JWT candidate probes

These isolated manifests and exact Cargo locks are research evidence, not SDK
workspace members or production dependencies. They were generated and executed
outside the repository on 2026-10-28, then copied here with only repository
Taplo whitespace normalization so dependency, compiler, and advisory claims can
be reproduced from the reviewed change.

## Integrity

| Probe | `Cargo.lock` SHA-256 |
| --- | --- |
| `affinidi` | `2de336e284e004c35a9dcc34f6ffcbad1295f60217676935b9cc6b34db8b543b` |
| `payload-minimal` | `5ff053402c43bfffff32fe2077dbf24fa2eda96026ea49149a48a67dc3257b64` |
| `payload-default` | `cd536a0c62b60460737dd3723bd84986a9793b19ea8b396225e639e7b835d526` |
| `ssi` | `9f5ea0b2cf65580941fe08d76ebb04039e1021f9d2edf87ac2e24717fbf0885d` |
| `reallyme` | `2999b7d36dedd472a4658a1d54a6f03ca05dec746daa8c8a8b9a49893c03c84a` |
| `bh` | `0226f6d11c65d3ad506c2be3f9047e44e286ba5308b85fd289021cdeb96f1d8e` |

## Exact commands

Run from the repository root. `PROBE_ROOT` is a task-specific variable and the
commands do not modify the SDK workspace or its lockfile.

```bash
PROBE_ROOT=openspec/changes/qualify-sd-jwt-engine/evidence/probes

shasum -a 256 \
  "$PROBE_ROOT"/affinidi/Cargo.lock \
  "$PROBE_ROOT"/payload-minimal/Cargo.lock \
  "$PROBE_ROOT"/payload-default/Cargo.lock \
  "$PROBE_ROOT"/ssi/Cargo.lock \
  "$PROBE_ROOT"/reallyme/Cargo.lock \
  "$PROBE_ROOT"/bh/Cargo.lock

for probe in affinidi payload-minimal payload-default ssi reallyme bh; do
  cargo +1.98.1 metadata --locked \
    --manifest-path "$PROBE_ROOT/$probe/Cargo.toml" \
    --format-version 1
  cargo +1.98.1 tree --locked \
    --manifest-path "$PROBE_ROOT/$probe/Cargo.toml" \
    --edges normal,build
  cargo +1.98.1 check --locked \
    --manifest-path "$PROBE_ROOT/$probe/Cargo.toml"
done

for probe in affinidi payload-minimal ssi; do
  cargo +1.89.0 check --locked \
    --manifest-path "$PROBE_ROOT/$probe/Cargo.toml"
done

cargo +1.98.1 install --locked --version 0.22.2 \
  --root /tmp/sdk-rust-489-tools cargo-audit

for probe in affinidi payload-minimal ssi reallyme bh; do
  /tmp/sdk-rust-489-tools/bin/cargo-audit audit \
    --file "$PROBE_ROOT/$probe/Cargo.lock" \
    --deny warnings --no-yanked
done

for mapping in \
  'affinidi:affinidi-sd-jwt' \
  'payload-minimal:sd-jwt-payload' \
  'ssi:ssi-sd-jwt' \
  'reallyme:reallyme-sd-jwt' \
  'bh:bh-sd-jwt'; do
  probe=${mapping%%:*}
  package=${mapping#*:}
  manifest=$(cargo +1.98.1 metadata --locked \
    --manifest-path "$PROBE_ROOT/$probe/Cargo.toml" \
    --format-version 1 | jq -r ".packages[] | select(.name == \"$package\") | .manifest_path")
  rg 'unsafe|extern "C"|#\[link|std::ffi|libc::' "$(dirname "$manifest")/src" || true
done
```

Rust 1.89 was not run for `reallyme` because its manifest rejects the primary
1.98.1 compiler, nor for `bh` because license/native/network coupling already
disqualified it. The default IOTA probe was measured only to compare feature
cost; the minimal probe is the adoption candidate. Advisory results use
RustSec database revision
`40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`.

## Recorded results

| Probe | Registry packages | Build scripts | Native or metadata `links` | Rust 1.98.1 | Rust 1.89 | RustSec 0.22.2 with warnings denied |
| --- | ---: | ---: | --- | --- | --- | --- |
| `affinidi` | 36 | 9 | none | pass | fail: package declares 1.95 | pass |
| `payload-minimal` | 46 | 11 | none | pass | pass | pass |
| `payload-default` | 56 | 13 | none | not repeated | not repeated | not repeated |
| `ssi` | 218 | 28 | `wasm-bindgen` metadata | pass | pass | fail: RUSTSEC-2023-0126, RUSTSEC-2026-0255, six unmaintained warnings |
| `reallyme` | 55 | 13 | `defmt` metadata | fail: package declares 1.99 | not run | pass |
| `bh` | 261 | 44 | OpenSSL, AWS-LC, ring, wasm-bindgen | not run after disqualification | not run | fail: two unmaintained warnings |

Registry yank state is not asserted. The repository's `SDK-LIM-010` records
that this environment lacks authoritative yank evidence.
