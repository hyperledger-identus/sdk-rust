# Design

Delete only the temporary `pull_request` mapping. Keep the base-context event,
exact base checkout, disabled persisted credentials, exact head-object import,
read-only permissions, and all policy steps unchanged. Change the source test
to reject reintroduction of the legacy event.

The PR itself is the natural canary: because protected `develop` now contains
`pull_request_target`, GitHub loads that base-owned workflow for #432. Merge is
allowed only if its exact-head `pull-request-policy` status is successful.

Rollback re-adds the legacy event only if the trusted event cannot emit the
required context; such a rollback is a new time-bounded incident change.
