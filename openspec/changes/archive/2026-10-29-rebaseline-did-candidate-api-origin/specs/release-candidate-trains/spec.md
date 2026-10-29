## ADDED Requirements

### Requirement: Prepublication API-origin rebaselines are exact and additive

Before the first candidate is published, the SDK SHALL permit rebaselining its
committed API origin only through an issue-linked exact-source receipt that binds old and
new generated API hashes, proves an empty removal set and a closed expected
addition set, retains unchanged package/version/feature/dependency/publication
state, and passes normal candidate qualification without initialization mode.
The rebaseline SHALL remain a first-candidate origin rather than a stable
SemVer or released-predecessor claim.

#### Scenario: an already-delivered additive API is missing from the origin

- **WHEN** exact generated API comparison contains no removal or changed item
  and additions equal the reviewed allowlist
- **THEN** the SDK records the immutable classification and may replace the
  stale unpublished origin before rerunning normal qualification

#### Scenario: the diff is breaking, ambiguous, or broader than reviewed

- **WHEN** any API item is removed or changed, an unexpected addition appears,
  the source revision differs, or publication has already occurred
- **THEN** the rebaseline fails and requires a separate compatibility decision
