## ADDED Requirements

### Requirement: DID engineering qualification is distinct from publication authority

The DID readiness surface SHALL separately report exact-revision engineering
qualification, downstream adoption, release authorization, registry
publication, stable API, and `main` promotion. Completion of one fact SHALL NOT
imply any other fact.

#### Scenario: exact slow and downstream adoption evidence are complete

- **WHEN** the exact SDK candidate slow run passes and a named downstream
  consumer merges the same immutable revision with compatibility evidence
- **THEN** the handbook reports engineering qualification and adoption as
  complete while retaining any administrator-owned publication, tag, stable
  API, or promotion gate as pending

#### Scenario: a human reviews the candidate

- **WHEN** review is requested
- **THEN** the review surface presents the code diff, acceptance behavior,
  evidence limitations, ownership boundaries, and exact identities rather than
  asking the reviewer to approve an opaque receipt hash
