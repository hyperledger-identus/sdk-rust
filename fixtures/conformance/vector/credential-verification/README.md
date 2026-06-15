# Credential Verification Vector Fixtures

These fixtures define Docker-free credential and presentation verification
negative cases for `identus-credentials`, `identus-presentations`, and
`identus-trust`.

The first matrix is synthetic and redacted. It describes expected typed error
families without embedding real credentials, signatures, private keys, bearer
tokens, or production identifiers. Protocol and format crates must preserve the
stable `case_id` values when they replace placeholders with executable JWT VC,
SD-JWT VC, W3C VC, AnonCreds, OpenBadges, or mdoc vectors.
