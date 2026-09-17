# Verification

The following checks passed at reviewed implementation head
`2d08783b2df1237de39fd9fbdc0f9800063833ec`:

```text
node --check scripts/factory-tools/delivery.mjs
node --test scripts/tests/factory-operations.mjs
  48 passed, 0 failed
scripts/factory check
./bootstrap.sh --check
scripts/factory delivery merge-pr --pr 321 \
  --expect-head 1ae7a53b55984d6f50184458113bd3facc376620 \
  --body-file <private-exact-merge-body> --execute
  receipt retained; no merge performed
git diff --check
```

No Nix/Rust slow closure was repeated because the repair changes only a
repository-local JavaScript timestamp predicate and its tests. PR exact-head
`fast` remains mandatory; the native weekly slow line retains production and
release-promotion responsibility.
