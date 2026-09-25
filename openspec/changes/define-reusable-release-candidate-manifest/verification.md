# Verification

## Exact clean candidate

At implementation head `a0ff1196b7d5959c65e70350298241eb61cea105` with
Rust/Cargo `1.98.1`:

```text
scripts/prepare-did-candidate.py --root . --output <external> \
  --revision a0ff1196b7d5959c65e70350298241eb61cea105
  sourceDirty: false
  twoPassByteIdentical: true
  12 profile commands passed
  elapsedSeconds: 42.259

identus-did-0.1.0-rc.1.crate
  bytes: 97727
  sha256: ec9ed5be2f8d716d1395cdea347ec71f19def355617579a7ad26cfc14681f7a7

identus-did-resolver-http-0.1.0-rc.1.crate
  bytes: 18999
  sha256: 875801cbcd2bb5b1e378ca4f84903f1f58949da7447cea7e586f172845eb8434
```

The matrix covers default and no-default profiles for both packages plus
resolver all-feature and explicit `openapi` profiles, each through `cargo
check` and `cargo test`. The extracted closure patches only the candidate DID
package; `identus-core` and `identus-derive` resolve at exact published
`=0.1.0-rc.1` requirements.

## Policy, compatibility and factory

```text
python3 scripts/check-release-candidates.py .
python3 scripts/tests/release-candidates.py
  closed contract and mutation cases passed

python3 scripts/check-release-train.py .
python3 scripts/tests/release-train.py
python3 scripts/check-crypto-candidate.py .
python3 scripts/tests/crypto-candidate.py
  existing first-train contracts passed unchanged

cargo fmt --all -- --check
cargo test --locked -p identus-did -p identus-did-resolver-http --all-features
  passed

scripts/check-factory.sh .
scripts/tests/factory-contract.sh
  structure and synthetic fixture passed

git diff --check
python3 -m py_compile scripts/check-release-candidates.py \
  scripts/prepare-did-candidate.py scripts/tests/release-candidates.py
  passed
```

The candidate command allowlist has direct positive and negative mutation
tests. The archive boundary test rejects a path-traversing member and the
scratch boundary test rejects repository-contained staging.

## Unchanged surfaces

No Rust implementation, canonical DID version/publish state, Cargo lockfile,
published crypto manifest/descriptor, publisher, workflow, credential,
registry state, tag, release, consumer repository or GitHub setting changed.
