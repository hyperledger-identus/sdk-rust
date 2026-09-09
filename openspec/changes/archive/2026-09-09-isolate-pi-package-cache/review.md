# Local review

- **Review date:** 2026-09-10
- **Review angle:** path safety, reproducibility, concurrency, privacy and
  recovery
- **Scope:** exact implementation diff after planning commit `8c06a67`
- **Result:** passed after findings were resolved

## Resolved findings

1. Exact top-level package declarations still allowed transitive npm ranges to
   resolve differently on a fresh machine. The factory now tracks an exact npm
   manifest and lock, validates registry origins and SHA-512 integrity records,
   keys the cache by the lock digest and populates with `npm ci`.
2. Creating a temporary symlink and renaming it over `.pi/npm` could replace a
   path created between inspection and rename. Link creation now targets the
   final path directly, fails on `EEXIST` and accepts only a concurrent link to
   the same verified cache.
3. A post-promotion verification error could report that the vanished staging
   path had been retained. Promotion and verification are now separated, and
   the diagnostic identifies whichever exact state actually remains.
4. The first isolated Nix fixture copied the old required-file inventory and
   therefore omitted the new cache module. The fixture inventory now matches
   the production checker; the isolated factory derivation passes.

## Security and privacy review

- Cache and link locations derive only from canonical Git worktree paths and
  exact tracked inputs; no arbitrary environment override is accepted.
- Existing directories, broken/wrong links, symlinked storage ancestry,
  malformed package sources, version drift and incomplete markers fail closed.
- `npm ci` uses the tracked lock, registry HTTPS URLs and integrity hashes with
  lifecycle scripts disabled. `npm audit --omit=dev --package-lock-only`
  reports zero vulnerabilities.
- The implementation reads no Pi authentication, session, transcript, prompt,
  provider or model file and performs no user configuration mutation.
- Cleanup is limited to a losing initializer's uniquely owned staging path.
  Existing and failed operator state is retained for explicit recovery.

## Remaining limitations

- Pi 0.84.2 still has no dedicated project-package storage override, so the
  supported bootstrap uses an ignored symlink at Pi's hard-coded path.
- Complete-cache retention is manual; no destructive pruning policy is
  activated.
- A locally privileged process can mutate local packages after verification;
  this cache contract is reproducible tooling and path hygiene, not a sandbox
  against a compromised host.
- Rust, targets, consumers, `main`, release and publication are unchanged.

No unresolved blocking finding remains.
