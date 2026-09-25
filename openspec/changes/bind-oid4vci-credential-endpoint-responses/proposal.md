# Bind OID4VCI Credential Endpoint responses

## Why

Issue #364 constructs a Credential Request from exact correlated Authorization
Code authority, but the current response API borrows a reusable request and
accepts only immediate success. A wallet needs one consuming transition that
classifies the Final immediate, deferred, and payload-error envelopes without
replaying the same typed request.

## What changes

- add one composed response-limit policy and one closed response-outcome enum;
- consume `JwtCredentialRequest` before classifying exact status/media/body;
- reuse the existing immediate, deferred-core, and Credential Error parsers;
- retain request proof count in every bound outcome and enforce it on immediate
  credentials;
- preserve the borrowed immediate-only compatibility method;
- append one static invalid-status diagnostic and update governing evidence;
- hand authority-preserving deferred continuation to issue #368.

## Non-goals

No HTTP execution or provenance, authorization-error/`WWW-Authenticate`
parsing, TLS, token validation/storage, proof generation, retry/recovery,
polling, credential verification/storage, consumer, release, chain, Midnight,
or product behavior.
