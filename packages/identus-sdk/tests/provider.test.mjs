import assert from "node:assert/strict";
import { test } from "node:test";

import { DidError } from "../dist/public.js";
import { retryableSingleton } from "../dist/internal/initialization.js";
import { createRustDidProvider } from "../dist/internal/provider.js";

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
    bindingApiVersion: () => 1,
    isParseError: (error) => error?.kind === "parse",
    parseDid: () => {
      throw new Error("parseDid mock is missing");
    },
    parseDidUrl: () => {
      throw new Error("parseDidUrl mock is missing");
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
      binding({ bindingApiVersion: () => 2 }),
      "node-wasm",
    ),
    /DID binding contract failed/u,
  );
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
