# harden-crypto-text-encoders

Issue #298 replaces the unbounded public `HexStr::from` and
`Base64UrlStrNoPad::from` compatibility surface with bounded fallible byte
construction while preserving canonical text, parsing, and internal fixed-size
key encoding.
