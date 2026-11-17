import assert from "node:assert/strict";
import { test } from "node:test";

import { DidError } from "../dist/public.js";
import { retryableSingleton } from "../dist/internal/initialization.js";
import {
  createRustDidProvider,
  createRustDidQueryProvider,
} from "../dist/internal/provider.js";

function owned(fields) {
  let freeCount = 0;
  return {
    value: {
      ...fields,
      free() {
        freeCount += 1;
      },
    },
    freeCount() {
      return freeCount;
    },
  };
}

function binding(overrides = {}) {
  return {
    bindingApiVersion: () => 3,
    isParseError: (error) => error?.kind === "parse",
    parseDid: () => {
      throw new Error("parseDid mock is missing");
    },
    parseDidUrl: () => {
      throw new Error("parseDidUrl mock is missing");
    },
    parseDidDocument: () => {
      throw new Error("parseDidDocument mock is missing");
    },
    prepareDidResolution: () => {
      throw new Error("prepareDidResolution mock is missing");
    },
    prepareDidDereferencing: () => {
      throw new Error("prepareDidDereferencing mock is missing");
    },
    parseDidResolutionResult: () => {
      throw new Error("parseDidResolutionResult mock is missing");
    },
    parseDidDereferencingResult: () => {
      throw new Error("parseDidDereferencingResult mock is missing");
    },
    ...overrides,
  };
}

test("projects and freezes a DID before freeing the raw view", () => {
  const raw = owned({
    value: "did:example:123",
    method: "example",
    methodSpecificId: "123",
  });
  const provider = createRustDidProvider(
    binding({ parseDid: () => raw.value }),
    "node-wasm",
  );

  const result = provider.parseDid("did:example:123");

  assert.deepEqual(result, {
    serialized: "did:example:123",
    method: "example",
    methodSpecificId: "123",
  });
  assert.equal(Object.isFrozen(result), true);
  assert.equal(raw.freeCount(), 1);
  assert.equal("free" in result, false);
});

test("preserves absent and empty DID URL components", () => {
  const absent = owned({
    value: "did:example:123",
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    path: "",
    query: undefined,
    fragment: undefined,
  });
  const empty = owned({
    value: "did:example:123?#",
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    path: "",
    query: "",
    fragment: "",
  });
  const queue = [absent.value, empty.value];
  const provider = createRustDidProvider(
    binding({ parseDidUrl: () => queue.shift() }),
    "browser-wasm",
  );

  assert.deepEqual(provider.parseDidUrl("did:example:123"), {
    serialized: "did:example:123",
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    path: "",
    query: undefined,
    fragment: undefined,
  });
  assert.deepEqual(provider.parseDidUrl("did:example:123?#"), {
    serialized: "did:example:123?#",
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    path: "",
    query: "",
    fragment: "",
  });
  assert.equal(absent.freeCount(), 1);
  assert.equal(empty.freeCount(), 1);
});

test("maps and frees a known parse error without reflecting input", () => {
  let freeCount = 0;
  const rawError = {
    kind: "parse",
    code: "did.invalid_did",
    free() {
      freeCount += 1;
    },
  };
  const secret = "did:EXAMPLE:do-not-reflect";
  const provider = createRustDidProvider(
    binding({ parseDid: () => { throw rawError; } }),
    "node-wasm",
  );

  assert.throws(
    () => provider.parseDid(secret),
    (error) => {
      assert.equal(error instanceof DidError, true);
      assert.equal(error.code, "did.invalid_did");
      assert.equal(error.message, "DID value is invalid");
      assert.equal(error.stack, undefined);
      assert.equal(JSON.stringify(error).includes(secret), false);
      return true;
    },
  );
  assert.equal(freeCount, 1);
});

test("frees an invalid projection exactly once and sanitizes the failure", () => {
  let freeCount = 0;
  const raw = {
    get value() {
      throw new Error("raw detail must not escape");
    },
    method: "example",
    methodSpecificId: "123",
    free() {
      freeCount += 1;
    },
  };
  const provider = createRustDidProvider(
    binding({ parseDid: () => raw }),
    "node-wasm",
  );

  assert.throws(
    () => provider.parseDid("did:example:123"),
    (error) => {
      assert.equal(error.name, "DidBindingError");
      assert.equal(error.message, "DID binding contract failed");
      assert.equal(error.message.includes("raw detail"), false);
      return true;
    },
  );
  assert.equal(freeCount, 1);
});

test("frees and sanitizes a malformed generated parse error", () => {
  let freeCount = 0;
  const rawError = {
    kind: "parse",
    get code() {
      throw new Error("generated error detail");
    },
    free() {
      freeCount += 1;
    },
  };
  const provider = createRustDidProvider(
    binding({ parseDid: () => { throw rawError; } }),
    "node-wasm",
  );

  assert.throws(
    () => provider.parseDid("did:example:123"),
    (error) => error.message === "DID binding contract failed"
      && !error.message.includes("generated error detail"),
  );
  assert.equal(freeCount, 1);
});

test("isolates observer failure after ownership is released", () => {
  const raw = owned({
    value: "did:example:123",
    method: "example",
    methodSpecificId: "123",
  });
  let observed;
  const provider = createRustDidProvider(
    binding({ parseDid: () => raw.value }),
    "browser-wasm",
    {
      observer(event) {
        assert.equal(raw.freeCount(), 1);
        observed = event;
        throw new Error("observer failure");
      },
    },
    { now: (() => { const values = [10, 15]; return () => values.shift(); })() },
  );

  assert.equal(provider.parseDid("did:example:123").serialized, "did:example:123");
  assert.deepEqual(observed, {
    operation: "did.parse",
    route: "browser-wasm",
    result: "success",
    mappingId: "did.value.typescript.legacy-v1",
    errorCode: undefined,
    durationMs: 5,
  });
  assert.equal(Object.isFrozen(observed), true);
});

test("sanitizes unknown binding failures and rejects API drift", () => {
  const secret = "generated failure containing caller material";
  const provider = createRustDidProvider(
    binding({ parseDid: () => { throw new Error(secret); } }),
    "node-wasm",
  );

  assert.throws(
    () => provider.parseDid("did:example:123"),
    (error) => error.message === "DID binding contract failed"
      && !error.message.includes(secret),
  );
  assert.throws(
    () => createRustDidProvider(
      binding({ bindingApiVersion: () => 1 }),
      "node-wasm",
    ),
    /DID binding contract failed/u,
  );
});

function assertDeepFrozen(root) {
  const pending = [root];
  const visited = new Set();
  while (pending.length > 0) {
    const value = pending.pop();
    if (value === null || typeof value !== "object" || visited.has(value)) continue;
    visited.add(value);
    assert.equal(Object.isFrozen(value), true);
    pending.push(...Object.values(value));
  }
}

test("projects a complete DID document as strict deeply frozen values", () => {
  const serialized = JSON.stringify({
    "@context": [
      "https://www.w3.org/ns/did/v1",
      { example: "https://example.com/ns#" },
    ],
    id: "did:example:123",
    controller: ["did:example:123", "did:example:controller"],
    alsoKnownAs: ["https://example.com/profile"],
    verificationMethod: [{
      id: "did:example:123#key-1",
      type: "JsonWebKey2020",
      controller: "did:example:123",
      publicKeyJwk: { kty: "OKP", crv: "Ed25519", x: "public" },
      expires: "2030-01-01T00:00:00Z",
    }],
    authentication: [
      "did:example:123#key-1",
      {
        id: "did:example:123#key-2",
        type: "Multikey",
        controller: "did:example:123",
        publicKeyMultibase: "z6MkiTBz1y",
      },
    ],
    assertionMethod: ["did:example:123#key-1"],
    keyAgreement: ["did:example:123#key-1"],
    capabilityInvocation: ["did:example:123#key-1"],
    capabilityDelegation: ["did:example:123#key-1"],
    service: [{
      id: "did:example:123#messages",
      type: ["DIDCommMessaging", "ExampleInbox"],
      serviceEndpoint: [
        "https://inbox.example/123",
        { uri: "https://backup.example/123", accept: ["didcomm/v2"] },
      ],
      priority: 1,
    }],
    "example:profile": { locale: "en", features: ["identity"] },
  });
  const raw = owned({ serialized, id: "did:example:123" });
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => raw.value }),
    "node-wasm",
  );

  const result = provider.parseDidDocument("caller bytes are Rust-owned");

  assert.equal(result.serialized, serialized);
  assert.deepEqual(result.id, {
    serialized: "did:example:123",
    method: "example",
    methodSpecificId: "123",
  });
  assert.equal(Array.isArray(result.context), true);
  assert.equal(Array.isArray(result.controller), true);
  assert.equal(result.verificationMethod[0].properties.publicKeyJwk.kty, "OKP");
  assert.deepEqual(result.authentication[0], {
    kind: "reference",
    reference: "did:example:123#key-1",
  });
  assert.equal(result.authentication[1].kind, "embedded");
  assert.equal(result.authentication[1].method.properties.publicKeyMultibase, "z6MkiTBz1y");
  assert.equal(Array.isArray(result.service[0].type), true);
  assert.equal(Array.isArray(result.service[0].serviceEndpoint), true);
  assert.equal(result.service[0].properties.priority, 1);
  assert.equal(result.properties["example:profile"].locale, "en");
  assertDeepFrozen(result);
  assert.equal(raw.freeCount(), 1);
  assert.equal("free" in result, false);
});

test("preserves scalar document cardinality and absent optional fields", () => {
  const serialized = JSON.stringify({
    "@context": "https://www.w3.org/ns/did/v1",
    id: "did:example:123",
    controller: "did:example:controller",
    service: [{
      id: "did:example:123#profile",
      type: "Profile",
      serviceEndpoint: { origins: ["https://example.com"] },
    }],
  });
  const raw = owned({ serialized, id: "did:example:123" });
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => raw.value }),
    "browser-wasm",
  );

  const result = provider.parseDidDocument(serialized);

  assert.equal(result.context, "https://www.w3.org/ns/did/v1");
  assert.equal(Array.isArray(result.controller), false);
  assert.equal(result.controller.serialized, "did:example:controller");
  assert.equal(Array.isArray(result.service[0].type), false);
  assert.equal(Array.isArray(result.service[0].serviceEndpoint), false);
  assert.equal(result.alsoKnownAs, undefined);
  assert.equal(result.verificationMethod, undefined);
  assert.deepEqual(result.properties, {});
  assertDeepFrozen(result);
  assert.equal(raw.freeCount(), 1);
});

test("rejects unsafe integer spellings without rounding and frees once", () => {
  const markers = [
    "9007199254740993",
    "9007199254740993.0",
    "9.007199254740993e15",
  ];
  const raws = markers.map((marker) => owned({
    serialized: `{"id":"did:example:123","unsafe":${marker}}`,
    id: "did:example:123",
  }));
  const queue = raws.map((raw) => raw.value);
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => queue.shift() }),
    "node-wasm",
  );

  for (const marker of markers) {
    assert.throws(
      () => provider.parseDidDocument("caller-secret-document"),
      (error) => {
        assert.equal(error instanceof DidError, true);
        assert.equal(error.code, "adapter.unsupported_lossy_mapping");
        assert.equal(error.message, "DID document cannot be projected safely");
        assert.equal(error.stack, undefined);
        assert.equal(JSON.stringify(error).includes(marker), false);
        return true;
      },
    );
  }
  for (const raw of raws) assert.equal(raw.freeCount(), 1);
});

test("maps a Rust document rejection with stable redaction and exact disposal", () => {
  let freeCount = 0;
  const rawError = {
    kind: "parse",
    code: "did.invalid_document",
    free() {
      freeCount += 1;
    },
  };
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => { throw rawError; } }),
    "node-wasm",
  );

  assert.throws(
    () => provider.parseDidDocument("caller-secret-document"),
    (error) => error instanceof DidError
      && error.code === "did.invalid_document"
      && error.message === "DID document is invalid"
      && error.stack === undefined
      && !JSON.stringify(error).includes("caller-secret-document"),
  );
  assert.equal(freeCount, 1);
});

test("fails closed on malformed or excessive generated document wire", () => {
  const malformed = owned({ serialized: "[]", id: "did:example:123" });
  let nested = "true";
  for (let index = 0; index < 66; index += 1) {
    nested = `{"n":${nested}}`;
  }
  const excessive = owned({
    serialized: `{"id":"did:example:123","deep":${nested}}`,
    id: "did:example:123",
  });
  const emptyCardinality = owned({
    serialized: "{\"id\":\"did:example:123\",\"controller\":[]}",
    id: "did:example:123",
  });
  const queue = [malformed.value, excessive.value, emptyCardinality.value];
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => queue.shift() }),
    "node-wasm",
  );

  for (const input of ["malformed", "excessive", "empty-cardinality"]) {
    assert.throws(
      () => provider.parseDidDocument(input),
      (error) => error.name === "DidBindingError"
        && error.message === "DID binding contract failed"
        && error.stack === undefined,
    );
  }
  assert.equal(malformed.freeCount(), 1);
  assert.equal(excessive.freeCount(), 1);
  assert.equal(emptyCardinality.freeCount(), 1);
});

test("contains prototype-looking extension names as frozen own data", () => {
  const serialized = "{\"id\":\"did:example:123\",\"__proto__\":{\"polluted\":true}}";
  const raw = owned({ serialized, id: "did:example:123" });
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => raw.value }),
    "node-wasm",
  );

  const result = provider.parseDidDocument(serialized);

  assert.equal(Object.hasOwn(result.properties, "__proto__"), true);
  assert.equal(result.properties.__proto__.polluted, true);
  assert.equal({}.polluted, undefined);
  assertDeepFrozen(result);
  assert.equal(raw.freeCount(), 1);
});

test("observes document projection without document data", () => {
  const raw = owned({
    serialized: "{\"id\":\"did:example:123\"}",
    id: "did:example:123",
  });
  const observations = [];
  const provider = createRustDidProvider(
    binding({ parseDidDocument: () => raw.value }),
    "browser-wasm",
    { observer: (value) => observations.push(value) },
    { now: (() => { const values = [3, 8]; return () => values.shift(); })() },
  );

  provider.parseDidDocument("caller-secret-document");

  assert.deepEqual(observations, [{
    operation: "did-document.parse",
    route: "browser-wasm",
    result: "success",
    mappingId: "did.document.typescript.legacy-v1",
    errorCode: undefined,
    durationMs: 5,
  }]);
  assert.equal(JSON.stringify(observations).includes("caller-secret-document"), false);
});

test("validates JavaScript input before entering the binding", () => {
  let calls = 0;
  const provider = createRustDidProvider(
    binding({ parseDid: () => { calls += 1; } }),
    "node-wasm",
  );

  assert.throws(() => provider.parseDid(42), /DID input must be a string/u);
  assert.equal(calls, 0);
});

test("initialization is singleton and retries only after a redacted failure", async () => {
  let calls = 0;
  let fail = false;
  const initialize = retryableSingleton(async () => {
    calls += 1;
    if (fail) throw new Error("generated initialization detail");
  });

  await Promise.all([initialize(), initialize()]);
  assert.equal(calls, 1);
  await initialize();
  assert.equal(calls, 1);

  let retryCalls = 0;
  fail = true;
  const retry = retryableSingleton(async () => {
    retryCalls += 1;
    if (retryCalls === 1) throw new Error("generated initialization detail");
  });
  await assert.rejects(
    retry(),
    (error) => error.name === "DidBindingInitializationError"
      && error.message === "DID binding initialization failed"
      && error.stack === undefined,
  );
  await retry();
  assert.equal(retryCalls, 2);
});

test("explicit rollback selection never falls through after Rust rejection", () => {
  let legacyCalls = 0;
  const legacy = Object.freeze({
    parseDid(value) {
      legacyCalls += 1;
      return Object.freeze({ serialized: value, method: "legacy", methodSpecificId: "selected" });
    },
    parseDidUrl() {
      throw new Error("unused legacy URL path");
    },
  });
  const rawError = {
    kind: "parse",
    code: "did.invalid_did",
    free() {},
  };
  const rust = createRustDidProvider(
    binding({ parseDid: () => { throw rawError; } }),
    "node-wasm",
  );
  const select = (route) => route === "rust" ? rust : legacy;

  assert.throws(() => select("rust").parseDid("invalid"), DidError);
  assert.equal(legacyCalls, 0);
  assert.equal(select("legacy").parseDid("legacy-value").method, "legacy");
  assert.equal(legacyCalls, 1);
});

test("resolves through one injected host after closing request ownership", async () => {
  const request = owned({
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    options: JSON.stringify({ noCache: false, example: 1 }),
  });
  const serialized = JSON.stringify({
    didResolutionMetadata: {},
    didDocument: { id: "did:example:123" },
    didDocumentMetadata: {},
  });
  const result = owned({ serialized, kind: "success" });
  let hostCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidResolution(did, options) {
        assert.equal(did, "did:example:123");
        assert.deepEqual(JSON.parse(options), { noCache: false, example: 1 });
        return request.value;
      },
      parseDidResolutionResult(did, raw) {
        assert.equal(did, "did:example:123");
        assert.equal(raw, serialized);
        return result.value;
      },
    }),
    "node-wasm",
    {
      resolver: {
        async resolve(value) {
          hostCalls += 1;
          assert.equal(request.freeCount(), 1);
          assert.equal(Object.isFrozen(value), true);
          assert.equal(Object.isFrozen(value.did), true);
          assert.equal(Object.isFrozen(value.options), true);
          assert.equal(value.options.noCache, false);
          assert.equal(value.options.properties.example, 1);
          return serialized;
        },
      },
    },
  );

  const resolved = await provider.resolveDid("did:example:123", {
    noCache: false,
    properties: { example: 1 },
  });

  assert.equal(resolved.kind, "success");
  assert.equal(resolved.document.id.serialized, "did:example:123");
  assert.equal(Object.isFrozen(resolved), true);
  assert.equal(hostCalls, 1);
  assert.equal(request.freeCount(), 1);
  assert.equal(result.freeCount(), 1);
});

test("rejects invalid nested resolution options before binding or host effects", async () => {
  let bindingCalls = 0;
  let hostCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidResolution() {
        bindingCalls += 1;
      },
    }),
    "node-wasm",
    {
      resolver: {
        async resolve() {
          hostCalls += 1;
          return "{}";
        },
      },
    },
  );

  const inherited = Object.create({ noCache: true });
  const forbiddenProto = JSON.parse(
    '{"properties":{"nested":{"__proto__":{"polluted":true}}}}',
  );
  const forbiddenConstructor = JSON.parse(
    '{"properties":{"nested":{"constructor":{"polluted":true}}}}',
  );
  const nonPlain = { properties: { nested: new Date(0) } };
  const unsafe = { properties: { nested: Number.MAX_SAFE_INTEGER + 1 } };
  const cycle = { properties: {} };
  cycle.properties.self = cycle.properties;
  for (const options of [
    inherited,
    forbiddenProto,
    forbiddenConstructor,
    nonPlain,
    unsafe,
    cycle,
  ]) {
    await assert.rejects(
      provider.resolveDid("did:example:123", options),
      (error) => error instanceof DidError
        && error.code === "did.invalid_resolution_options"
        && error.stack === undefined,
    );
  }
  assert.equal(bindingCalls, 0);
  assert.equal(hostCalls, 0);
  assert.equal({}.polluted, undefined);
});

test("maps invalid nested dereferencing options without invoking the binding", async () => {
  let bindingCalls = 0;
  let hostCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidDereferencing() {
        bindingCalls += 1;
      },
    }),
    "node-wasm",
    {
      dereferencer: {
        async dereference() {
          hostCalls += 1;
          return "{}";
        },
      },
    },
  );
  const options = JSON.parse(
    '{"properties":{"nested":{"prototype":{"polluted":true}}}}',
  );

  await assert.rejects(
    provider.dereferenceDidUrl("did:example:123#key-1", options),
    (error) => error instanceof DidError
      && error.code === "did.invalid_dereferencing_options"
      && error.stack === undefined,
  );
  assert.equal(bindingCalls, 0);
  assert.equal(hostCalls, 0);
  assert.equal({}.polluted, undefined);
});

test("bounds resolution host text by UTF-8 bytes before the WASM parse call", async () => {
  const exact = "é".repeat(262_144);
  const over = `${exact}a`;
  const requests = [0, 1].map(() => owned({
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    options: "{}",
  }));
  const serialized = JSON.stringify({
    didResolutionMetadata: {},
    didDocument: { id: "did:example:123" },
    didDocumentMetadata: {},
  });
  const result = owned({ serialized, kind: "success" });
  let response = exact;
  let parseCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidResolution: () => requests.shift().value,
      parseDidResolutionResult(did, raw) {
        parseCalls += 1;
        assert.equal(did, "did:example:123");
        assert.equal(raw, exact);
        return result.value;
      },
    }),
    "node-wasm",
    { resolver: { resolve: async () => response } },
  );

  assert.equal((await provider.resolveDid("did:example:123")).kind, "success");
  response = over;
  await assert.rejects(
    provider.resolveDid("did:example:123"),
    (error) => error instanceof DidError
      && error.code === "did.invalid_resolution_result",
  );
  assert.equal(parseCalls, 1);
  assert.equal(result.freeCount(), 1);
});

test("preserves a W3C problem as result data instead of throwing", async () => {
  const request = owned({
    did: "did:example:missing",
    method: "example",
    methodSpecificId: "missing",
    options: "{}",
  });
  const serialized = JSON.stringify({
    didResolutionMetadata: {
      error: {
        type: "https://www.w3.org/ns/did#NOT_FOUND",
        title: "Not found",
      },
    },
    didDocument: null,
    didDocumentMetadata: {},
  });
  const result = owned({ serialized, kind: "error" });
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidResolution: () => request.value,
      parseDidResolutionResult: () => result.value,
    }),
    "browser-wasm",
    { resolver: { resolve: async () => serialized } },
  );

  const resolved = await provider.resolveDid("did:example:missing");

  assert.equal(resolved.kind, "failure");
  assert.equal(resolved.document, null);
  assert.equal(resolved.resolutionMetadata.error.standardKind, "notFound");
  assert.equal(resolved.resolutionMetadata.error.title, "Not found");
  assert.equal(result.freeCount(), 1);
});

test("dereferences bounded JSON through the independent host effect", async () => {
  const request = owned({
    didUrl: "did:example:123#key-1",
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    path: "",
    query: undefined,
    fragment: "key-1",
    options: JSON.stringify({ verificationRelationship: "authentication" }),
  });
  const serialized = JSON.stringify({
    didUrlDereferencingMetadata: { contentType: "application/did+json" },
    content: { id: "did:example:123#key-1", type: "Multikey" },
    contentMetadata: { source: "fixture" },
  });
  const result = owned({ serialized, kind: "success" });
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidDereferencing: () => request.value,
      parseDidDereferencingResult: () => result.value,
    }),
    "node-wasm",
    { dereferencer: { dereference: async () => serialized } },
  );

  const dereferenced = await provider.dereferenceDidUrl(
    "did:example:123#key-1",
    { verificationRelationship: "authentication" },
  );

  assert.equal(dereferenced.kind, "success");
  assert.equal(dereferenced.content.id, "did:example:123#key-1");
  assert.equal(dereferenced.contentMetadata.properties.source, "fixture");
  assert.equal(request.freeCount(), 1);
  assert.equal(result.freeCount(), 1);
});

test("bounds dereferencing host text by UTF-8 bytes before the WASM parse call", async () => {
  const exact = "é".repeat(262_144);
  const over = `${exact}a`;
  const requests = [0, 1].map(() => owned({
    didUrl: "did:example:123#key-1",
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    path: "",
    query: undefined,
    fragment: "key-1",
    options: "{}",
  }));
  const serialized = JSON.stringify({
    didUrlDereferencingMetadata: {},
    content: { id: "did:example:123#key-1" },
    contentMetadata: {},
  });
  const result = owned({ serialized, kind: "success" });
  let response = exact;
  let parseCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidDereferencing: () => requests.shift().value,
      parseDidDereferencingResult(raw) {
        parseCalls += 1;
        assert.equal(raw, exact);
        return result.value;
      },
    }),
    "node-wasm",
    { dereferencer: { dereference: async () => response } },
  );

  assert.equal(
    (await provider.dereferenceDidUrl("did:example:123#key-1")).kind,
    "success",
  );
  response = over;
  await assert.rejects(
    provider.dereferenceDidUrl("did:example:123#key-1"),
    (error) => error instanceof DidError
      && error.code === "did.invalid_dereferencing_result",
  );
  assert.equal(parseCalls, 1);
  assert.equal(result.freeCount(), 1);
});

test("host rejection is redacted, terminal, and observed once", async () => {
  const request = owned({
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    options: "{}",
  });
  let calls = 0;
  const observations = [];
  const provider = createRustDidQueryProvider(
    binding({ prepareDidResolution: () => request.value }),
    "node-wasm",
    {
      resolver: {
        async resolve() {
          calls += 1;
          throw new Error("https://secret.example/did:example:123");
        },
      },
    },
    { observer: (event) => observations.push(event) },
  );

  await assert.rejects(
    provider.resolveDid("did:example:123"),
    (error) => error instanceof DidError
      && error.code === "did.resolution_host_failure"
      && error.stack === undefined
      && !error.message.includes("secret"),
  );
  assert.equal(calls, 1);
  assert.equal(request.freeCount(), 1);
  assert.deepEqual(observations.map((event) => event.errorCode), [
    "did.resolution_host_failure",
  ]);
  assert.equal(JSON.stringify(observations).includes("did:example:123"), false);
});

test("already-aborted operations never allocate or invoke the host", async () => {
  const controller = new AbortController();
  controller.abort();
  let bindingCalls = 0;
  let hostCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidResolution() {
        bindingCalls += 1;
      },
    }),
    "node-wasm",
    {
      resolver: {
        async resolve() {
          hostCalls += 1;
          return "{}";
        },
      },
    },
  );

  await assert.rejects(
    provider.resolveDid("did:example:123", undefined, {
      signal: controller.signal,
    }),
    (error) => error instanceof DidError && error.code === "did.operation_aborted",
  );
  assert.equal(bindingCalls, 0);
  assert.equal(hostCalls, 0);
});

test("query observer failure is terminal after result ownership closes", async () => {
  const request = owned({
    did: "did:example:123",
    method: "example",
    methodSpecificId: "123",
    options: "{}",
  });
  const serialized = JSON.stringify({
    didResolutionMetadata: {},
    didDocument: { id: "did:example:123" },
    didDocumentMetadata: {},
  });
  const result = owned({ serialized, kind: "success" });
  let observerCalls = 0;
  const provider = createRustDidQueryProvider(
    binding({
      prepareDidResolution: () => request.value,
      parseDidResolutionResult: () => result.value,
    }),
    "node-wasm",
    { resolver: { resolve: async () => serialized } },
    {
      observer() {
        observerCalls += 1;
        assert.equal(result.freeCount(), 1);
        throw new Error("observer detail");
      },
    },
  );

  await assert.rejects(
    provider.resolveDid("did:example:123"),
    (error) => error instanceof DidError && error.code === "did.observer_failure",
  );
  assert.equal(observerCalls, 1);
  assert.equal(request.freeCount(), 1);
  assert.equal(result.freeCount(), 1);
});
