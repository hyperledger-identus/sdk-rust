# Exact-diff factory and security review

- **Issue:** #333
- **Exact base:** `20e4f7ad6630cec476fbadf1b71bb2315aca7876`
- **Implementation head:** `8847fd96c86e0a88d9dd4ed79e068d4a7705f80f`
- **Result:** approved locally with no unresolved blocker

## Findings

1. **Resolved — rebased updates can no longer omit commits from local policy.**
   The pre-push hook derives its validation range from Git ancestry rather than
   accepting the remote feature SHA unconditionally. A divergent remote head
   selects the current fetched protected base, so every outgoing commit after
   `origin/develop` is revalidated.
2. **Accepted — ordinary updates retain the narrow range.** A remote feature
   head is selected only when it is both an ancestor of the outgoing head and
   a descendant of the current protected base. First pushes select the
   protected base directly.
3. **Accepted — ambiguous graph evidence fails closed.** Missing or malformed
   exact commit objects, an outgoing head that does not descend from
   `origin/develop`, and unrelated remote history stop the push before a
   network mutation. The resolver accepts only lowercase full object IDs and
   the fixed `refs/remotes/origin/develop` policy anchor.
4. **Accepted — authority remains local and bounded.** The resolver executes
   only fixed-argument Git object and ancestry queries. It does not fetch,
   interpret ref names from stdin, invoke a shell, alter hosted policy, or
   expand the contribution validator.
5. **Accepted — signature behavior is proven end to end.** A temporary Git
   graph shows that an unsigned protected-base advance is excluded while a
   signed rebased feature commit passes and an unsigned feature commit fails.
   Ephemeral SSH material is removed with the fixture and never enters the
   repository.
6. **Resolved — hermetic evidence initially lacked `ssh-keygen`.** The factory
   derivation now includes nixpkgs-pinned OpenSSH as test tooling. It creates
   no SDK dependency, public API, runtime, target, or product support promise.
7. **Accepted — regression and portability evidence is complete.** The focused
   resolver tests, 62-test operational suite, hermetic factory contract, full
   897-test workspace lane, feature combinations, formatting, documentation,
   Clippy, MSRV/portable checks, and repository linters pass. A non-fatal
   Darwin Nix fixup pipeline warning did not fail any derivation and is
   unrelated to the branch logic.

## Residual limitation

The hook intentionally trusts the contributor's fetched `origin/develop`; it
does not perform network synchronization. Repository synchronization and
delivery preflight remain responsible for freshness. Hosted exact-head CI is
still required before protected merge.

## Verdict

The slice closes the rebased-push validation gap without weakening normal
fast-forward ergonomics or expanding SDK/product scope. It is ready for guarded
OpenSpec archive, signed/DCO delivery, and exact-head hosted validation.
