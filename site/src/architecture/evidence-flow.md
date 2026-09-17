# Evidence and release flow

![Evidence flow from source to release](../diagrams/evidence-flow.svg)

The repository separates ordinary integration from production promotion:

- **Fast line:** one required Ubuntu build/lint/test/factory gate for pull
  requests into `develop`.
- **Slow line:** weekly/manual target, binding, fuzz, coverage, package,
  performance, and platform evidence on an exact protected revision.
- **Candidate gate:** deterministic packages, API/SemVer review, SBOMs,
  checksums, provenance, source closure, consumer evidence, and documentation.
- **Human release gate:** one release manager and a second maintainer approve
  the exact receipt before a tag or registry upload.

Documentation publication is evidence before the release decision. It does not
advance the candidate by itself.
