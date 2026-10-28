# identus-did-web

Portable, bounded `did:web` parsing and resolution over a caller-injected fetch
port. The crate derives exact HTTPS requests and validates returned DID
documents; applications retain DNS, TLS, redirect, proxy, timeout,
cancellation, cache, retry, privacy, telemetry, and SSRF policy.

The first profile accepts lowercase ASCII multi-label FQDNs, an optional
canonical `%3A` non-default port, and conservative DID-`idchar` path segments
without percent escapes or `~`.
It contains no HTTP client or runtime dependency.
