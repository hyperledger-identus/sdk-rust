// SPDX-License-Identifier: Apache-2.0

export const SDK_TS_DONOR_REVISION =
  "4bf86ebf69d5e96616a148e4c973f831f95fa38e";

const PAIRWISE_DID =
  "did:peer:2.Ez6LSoHkfN1Y4nK9RCjx7vopWsLrMGNFNgTNZgoCNQrTzmb1n.Vz6MknRZmapV7uYZQuZez9n9N3tQotjRN18UGS68Vcfo6gR4h.SeyJ0IjoiZG0iLCJzIjp7InVyaSI6Imh0dHBzOi8vZXhhbXBsZS5jb20vZW5kcG9pbnQiLCJyIjpbImRpZDpleGFtcGxlOnNvbWVtZWRpYXRvciNzb21la2V5Il0sImEiOltdfX0";
const PAIRWISE_DID_URL = `${PAIRWISE_DID}#key-2`;
const PAIRWISE_INPUT = Object.freeze({
  keys: Object.freeze([
    Object.freeze({
      purpose: "encryption",
      multibase: "z6LSoHkfN1Y4nK9RCjx7vopWsLrMGNFNgTNZgoCNQrTzmb1n",
    }),
    Object.freeze({
      purpose: "verification",
      multibase: "z6MknRZmapV7uYZQuZez9n9N3tQotjRN18UGS68Vcfo6gR4h",
    }),
  ]),
  services: Object.freeze([
    Object.freeze({
      type: "DIDCommMessaging",
      endpoint: Object.freeze({
        uri: "https://example.com/endpoint",
        accept: Object.freeze([]),
        routingKeys: Object.freeze(["did:example:somemediator#somekey"]),
      }),
    }),
  ]),
});
const DOCUMENT_LITERAL = `{"id":"${PAIRWISE_DID}"}`;
const RESOLUTION_LITERAL = JSON.stringify({
  didResolutionMetadata: { contentType: "application/did+ld+json" },
  didDocument: { id: PAIRWISE_DID },
  didDocumentMetadata: { versionId: "sdk-ts-8.1.4" },
});
const DEREFERENCING_LITERAL = JSON.stringify({
  didUrlDereferencingMetadata: { contentType: "application/did+ld+json" },
  content: { id: PAIRWISE_DID_URL, type: "Multikey" },
  contentMetadata: {},
});

export const DID_ADOPTION_JOURNEY = Object.freeze({
  id: "sdk-ts.did-facade-adoption.v1",
  donorRevision: SDK_TS_DONOR_REVISION,
  pairwiseDid: PAIRWISE_DID,
  pairwiseDidUrl: PAIRWISE_DID_URL,
  pairwiseInput: PAIRWISE_INPUT,
  documentLiteral: DOCUMENT_LITERAL,
  resolutionLiteral: RESOLUTION_LITERAL,
  dereferencingLiteral: DEREFERENCING_LITERAL,
  rustAddition: Object.freeze({
    profile: "numalgo4",
    input: Object.freeze({ hello: "world" }),
    expectedLongForm:
      "did:peer:4zQmb7xLdVY9TXx8oov5XgpGUmGELgqiAV2699s43i6Qdm3M:zQSJgiFTYiCHjQ9MktwNThRXM7a",
    legacyDisposition: "not-applicable",
  }),
});

function deepFreeze(value) {
  if (value === null || typeof value !== "object" || Object.isFrozen(value)) return value;
  for (const child of Object.values(value)) deepFreeze(child);
  return Object.freeze(value);
}

function didValue(serialized) {
  const separator = serialized.indexOf(":", 4);
  return Object.freeze({
    serialized,
    method: serialized.slice(4, separator),
    methodSpecificId: serialized.slice(separator + 1),
  });
}

function didUrlValue(serialized) {
  const fragmentAt = serialized.indexOf("#");
  const did = serialized.slice(0, fragmentAt);
  return Object.freeze({
    ...didValue(did),
    serialized,
    did,
    path: "",
    query: undefined,
    fragment: serialized.slice(fragmentAt + 1),
  });
}

function documentValue() {
  return deepFreeze({
    serialized: DOCUMENT_LITERAL,
    id: didValue(PAIRWISE_DID),
    context: undefined,
    controller: undefined,
    alsoKnownAs: undefined,
    verificationMethod: undefined,
    authentication: [{ kind: "reference", reference: `${PAIRWISE_DID}#key-2` }],
    assertionMethod: undefined,
    keyAgreement: [{ kind: "reference", reference: `${PAIRWISE_DID}#key-1` }],
    capabilityInvocation: undefined,
    capabilityDelegation: undefined,
    service: [{
      id: `${PAIRWISE_DID}#service-0`,
      type: "DIDCommMessaging",
      serviceEndpoint: {
        uri: "https://example.com/endpoint",
        routingKeys: ["did:example:somemediator#somekey"],
        accept: [],
      },
      properties: {},
    }],
    properties: {},
  });
}

function requireExact(actual, expected, operation) {
  if (actual !== expected) {
    throw Object.assign(new Error(`Legacy fixture rejects ${operation}`), {
      name: "DidError",
      code: "adapter.unsupported_lossy_mapping",
      stack: undefined,
    });
  }
}

/**
 * Fixed compatibility fixture for one reviewed SDK-TS 8.1.4 journey.
 * It intentionally rejects every value outside that fixture instead of
 * reimplementing parsing, resolution, or peer-DID semantics.
 */
export function createLegacyAdoptionFixture() {
  const calls = Object.create(null);
  const count = (operation) => { calls[operation] = (calls[operation] ?? 0) + 1; };
  const document = documentValue();
  const provider = Object.freeze({
    parseDid(value) {
      count("parseDid");
      requireExact(value, PAIRWISE_DID, "parseDid");
      return didValue(value);
    },
    parseDidUrl(value) {
      count("parseDidUrl");
      requireExact(value, PAIRWISE_DID_URL, "parseDidUrl");
      return didUrlValue(value);
    },
    parseDidDocument(value) {
      count("parseDidDocument");
      requireExact(value, DOCUMENT_LITERAL, "parseDidDocument");
      return document;
    },
    createPeerDidNumalgo2Compatibility(input) {
      count("createPeerDidNumalgo2Compatibility");
      requireExact(JSON.stringify(input), JSON.stringify(PAIRWISE_INPUT), "createPeerDidNumalgo2Compatibility");
      return deepFreeze({
        profile: "numalgo2-compatibility",
        did: didValue(PAIRWISE_DID),
        document,
      });
    },
    parsePeerDidNumalgo2Compatibility(value) {
      count("parsePeerDidNumalgo2Compatibility");
      requireExact(value, PAIRWISE_DID, "parsePeerDidNumalgo2Compatibility");
      return deepFreeze({
        profile: "numalgo2-compatibility",
        did: didValue(PAIRWISE_DID),
        document,
      });
    },
  });

  const createQueryProvider = async (effects) => Object.freeze({
    ...provider,
    async resolveDid(value) {
      count("resolveDid");
      requireExact(value, PAIRWISE_DID, "resolveDid");
      const serialized = await effects.resolver.resolve(Object.freeze({
        did: didValue(value),
        options: deepFreeze({
          accept: undefined,
          expandRelativeUrls: undefined,
          noCache: undefined,
          versionId: undefined,
          versionTime: undefined,
          properties: {},
        }),
        signal: undefined,
      }));
      requireExact(serialized, RESOLUTION_LITERAL, "resolution result");
      return deepFreeze({
        kind: "success",
        serialized,
        resolutionMetadata: { contentType: "application/did+ld+json", error: undefined, properties: {} },
        document,
        documentMetadata: {
          created: undefined,
          updated: undefined,
          deactivated: undefined,
          nextUpdate: undefined,
          versionId: "sdk-ts-8.1.4",
          nextVersionId: undefined,
          equivalentId: undefined,
          canonicalId: undefined,
          properties: {},
        },
      });
    },
    async dereferenceDidUrl(value) {
      count("dereferenceDidUrl");
      requireExact(value, PAIRWISE_DID_URL, "dereferenceDidUrl");
      const serialized = await effects.dereferencer.dereference(Object.freeze({
        didUrl: didUrlValue(value),
        options: deepFreeze({ accept: undefined, verificationRelationship: undefined, properties: {} }),
        signal: undefined,
      }));
      requireExact(serialized, DEREFERENCING_LITERAL, "dereferencing result");
      return deepFreeze({
        kind: "success",
        serialized,
        dereferencingMetadata: { contentType: "application/did+ld+json", error: undefined, properties: {} },
        content: { id: PAIRWISE_DID_URL, type: "Multikey" },
        contentMetadata: { properties: {} },
      });
    },
  });

  return Object.freeze({
    provider,
    createQueryProvider,
    calls: () => Object.freeze({ ...calls }),
    totalCalls: () => Object.values(calls).reduce((sum, value) => sum + value, 0),
  });
}

function assertionCounter(route) {
  let count = 0;
  return Object.freeze({
    equal(actual, expected, label) {
      count += 1;
      if (!Object.is(actual, expected)) {
        throw new Error(`DID adoption assertion failed: ${route}:${label}`);
      }
    },
    finish: () => count,
  });
}

export async function assertDidAdoptionJourney({ provider, createQueryProvider, route }) {
  const assertions = assertionCounter(route);
  const created = provider.createPeerDidNumalgo2Compatibility(PAIRWISE_INPUT);
  assertions.equal(created.did.serialized, PAIRWISE_DID, "created-did");
  assertions.equal(created.profile, "numalgo2-compatibility", "created-profile");
  assertions.equal(created.document.authentication?.[0]?.reference, `${PAIRWISE_DID}#key-2`, "authentication");
  assertions.equal(created.document.keyAgreement?.[0]?.reference, `${PAIRWISE_DID}#key-1`, "key-agreement");
  assertions.equal(created.document.service?.[0]?.type, "DIDCommMessaging", "service-type");
  assertions.equal(created.document.service?.[0]?.serviceEndpoint?.uri, "https://example.com/endpoint", "service-uri");

  const parsed = provider.parseDid(created.did.serialized);
  assertions.equal(parsed.serialized, PAIRWISE_DID, "parsed-did");
  const parsedUrl = provider.parseDidUrl(PAIRWISE_DID_URL);
  assertions.equal(parsedUrl.did, PAIRWISE_DID, "parsed-url-did");
  assertions.equal(parsedUrl.fragment, "key-2", "parsed-url-fragment");
  const parsedDocument = provider.parseDidDocument(DOCUMENT_LITERAL);
  assertions.equal(parsedDocument.id.serialized, PAIRWISE_DID, "document-id");
  const reparsed = provider.parsePeerDidNumalgo2Compatibility(PAIRWISE_DID);
  assertions.equal(reparsed.did.serialized, PAIRWISE_DID, "reparsed-peer-did");

  const pairwise = Object.freeze({ from: created.did.serialized, to: PAIRWISE_DID_URL });
  assertions.equal(Object.isFrozen(pairwise), true, "pairwise-frozen");
  assertions.equal(pairwise.from, PAIRWISE_DID, "pairwise-from");
  assertions.equal(pairwise.to, PAIRWISE_DID_URL, "pairwise-to");

  let resolvedRequest;
  let dereferencedRequest;
  const queryProvider = await createQueryProvider({
    resolver: { resolve: async (request) => { resolvedRequest = request; return RESOLUTION_LITERAL; } },
    dereferencer: { dereference: async (request) => { dereferencedRequest = request; return DEREFERENCING_LITERAL; } },
  });
  const resolved = await queryProvider.resolveDid(PAIRWISE_DID);
  assertions.equal(resolved.kind, "success", "resolution-kind");
  assertions.equal(resolved.document.id.serialized, PAIRWISE_DID, "resolution-document-id");
  assertions.equal(resolvedRequest.did.serialized, PAIRWISE_DID, "resolution-request-correlation");
  const dereferenced = await queryProvider.dereferenceDidUrl(PAIRWISE_DID_URL);
  assertions.equal(dereferenced.kind, "success", "dereferencing-kind");
  assertions.equal(dereferenced.content.id, PAIRWISE_DID_URL, "dereferencing-content-id");
  assertions.equal(dereferencedRequest.didUrl.serialized, PAIRWISE_DID_URL, "dereferencing-request-correlation");

  if (route === "legacy") {
    assertions.equal(DID_ADOPTION_JOURNEY.rustAddition.legacyDisposition, "not-applicable", "numalgo4-disposition");
    assertions.equal(typeof provider.createPeerDidNumalgo4, "undefined", "numalgo4-not-claimed");
  } else {
    const addition = provider.createPeerDidNumalgo4(DID_ADOPTION_JOURNEY.rustAddition.input);
    assertions.equal(addition.profile, "numalgo4", "numalgo4-profile");
    assertions.equal(addition.longForm.serialized, DID_ADOPTION_JOURNEY.rustAddition.expectedLongForm, "numalgo4-long-form");
  }

  return Object.freeze({ route, assertions: assertions.finish(), pairwise });
}

export async function runDidAdoptionRehearsal({
  target,
  selectDidProvider,
  createDidProvider,
  createDidQueryProvider,
}) {
  const legacy = createLegacyAdoptionFixture();
  const selectedLegacy = await selectDidProvider({ kind: "injected", provider: legacy.provider });
  const legacyResult = await assertDidAdoptionJourney({
    provider: selectedLegacy,
    createQueryProvider: legacy.createQueryProvider,
    route: "legacy",
  });
  const rustProvider = await selectDidProvider({ kind: "peer-wasm" });
  const rustResult = await assertDidAdoptionJourney({
    provider: rustProvider,
    createQueryProvider: createDidQueryProvider,
    route: "rust",
  });

  const callsBeforeFailure = legacy.totalCalls();
  let stableError;
  try {
    rustProvider.parseDid("did:EXAMPLE:must-not-fallback");
  } catch (error) {
    stableError = error;
  }
  if (stableError?.code !== "did.invalid_did" || legacy.totalCalls() !== callsBeforeFailure) {
    throw new Error(`DID adoption assertion failed: ${target}:terminal-selection`);
  }

  const rollback = await selectDidProvider({ kind: "injected", provider: legacy.provider });
  if (rollback !== legacy.provider || rollback.parseDid(PAIRWISE_DID).serialized !== PAIRWISE_DID) {
    throw new Error(`DID adoption assertion failed: ${target}:explicit-rollback`);
  }
  if (await createDidProvider() === rollback) {
    throw new Error(`DID adoption assertion failed: ${target}:provider-alias`);
  }

  return Object.freeze({
    target,
    donorRevision: SDK_TS_DONOR_REVISION,
    routes: Object.freeze([legacyResult, rustResult]),
    terminalFailure: stableError.code,
    rollback: "explicit",
    automaticFallback: false,
  });
}
