import assert from "node:assert/strict";
import { test } from "node:test";

import { DidError } from "../dist/public.js";
import {
  serializePeerDidNumalgo2Input,
  serializePeerDidNumalgo4Input,
} from "../dist/internal/peer-input.js";
import {
  projectPeerDidNumalgo2Compatibility,
  projectPeerDidNumalgo4,
} from "../dist/internal/peer-projection.js";
import {
  serializePrismCreateInput,
  serializePrismHistory,
} from "../dist/internal/prism-input.js";
import {
  projectPrismDid,
  projectPrismDidState,
} from "../dist/internal/prism-projection.js";

const SEED = 0x605f_0078;
const CASES = 2_048;
const VERIFICATION_KEY = "z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv";
const PRISM_KEY = "AuRZmOGBFxE4NblytDeE-2tkcLciyy3BccX7oAJlvIDu";

function generator(seed) {
  let state = seed >>> 0;
  return () => {
    state ^= state << 13;
    state ^= state >>> 17;
    state ^= state << 5;
    return state >>> 0;
  };
}

function boundedText(next, limit = 96) {
  const length = next() % limit;
  return Array.from({ length }, () => String.fromCharCode(0x20 + (next() % 95))).join("");
}

function numalgo4Candidate(next) {
  switch (next() % 8) {
    case 0:
      return { claim: boundedText(next) };
    case 1:
      return { nested: { values: [next() % 17, boundedText(next), null] } };
    case 2: {
      const value = { cycle: undefined };
      value.cycle = value;
      return value;
    }
    case 3:
      return Object.defineProperty({}, "accessor", { enumerable: true, get: () => "secret" });
    case 4:
      return { __proto__: { polluted: true }, value: boundedText(next) };
    case 5:
      return { unsafe: Number.MAX_SAFE_INTEGER + 1 };
    case 6:
      return Array.from({ length: next() % 140 }, () => boundedText(next, 8));
    default:
      return { id: `did:peer:${boundedText(next, 12)}` };
  }
}

function numalgo2Candidate(next) {
  const base = {
    keys: [{
      purpose: next() % 3 === 0 ? "unknown" : "verification",
      multibase: next() % 4 === 0 ? boundedText(next) : VERIFICATION_KEY,
    }],
    services: [{
      type: next() % 5 === 0 ? boundedText(next, 24) : "DIDCommMessaging",
      endpoint: {
        uri: `https://example.com/${next() % 97}`,
        accept: [boundedText(next, 24)],
        routingKeys: [`did:example:${next() % 31}#key`],
      },
      ...(next() % 7 === 0 ? { id: "bare-id" } : {}),
    }],
  };
  if (next() % 9 === 0) base.privateKey = boundedText(next);
  return base;
}

function assertStableIngressOutcome(run) {
  try {
    const serialized = run();
    assert.equal(typeof serialized, "string");
    assert.ok(Buffer.byteLength(serialized, "utf8") <= 24 * 1_024);
  } catch (error) {
    if (error instanceof DidError) {
      assert.ok([
        "did.peer.invalid_input_document",
        "did.peer.resource_limit",
        "did.peer.unsupported_purpose",
      ].includes(error.code));
    } else {
      assert.equal(error?.name, "DidBindingError");
    }
    assert.equal(error.stack, undefined);
  }
}

function prismCreateCandidate(next) {
  const key = {
    id: next() % 5 === 0 ? "" : `master-${next() % 17}`,
    usage: next() % 7 === 0 ? "unknown" : "master",
    curve: next() % 11 === 0 ? "unknown" : "secp256k1",
    publicKey: next() % 13 === 0 ? boundedText(next) : PRISM_KEY,
  };
  if (next() % 17 === 0) key.privateKey = boundedText(next);
  return {
    keys: [key],
    services: [{
      id: `service-${next() % 23}`,
      type: "LinkedDomains",
      serviceEndpoint: next() % 19 === 0
        ? Object.defineProperty({}, "secret", { enumerable: true, get: () => "value" })
        : { uri: `https://example.com/${next() % 97}`, weight: (next() % 50) / 7 },
    }],
  };
}

function prismHistoryCandidate(next) {
  switch (next() % 6) {
    case 0: return ["CgAB"];
    case 1: return ["="];
    case 2: return ["A"];
    case 3: return [boundedText(next)];
    case 4: return Object.defineProperty([], "0", { enumerable: true, get: () => "CgAB" });
    default: return Array.from({ length: next() % 32 }, () => "CgAB");
  }
}

function assertStablePrismIngress(run) {
  try {
    const serialized = run();
    assert.equal(typeof serialized, "string");
    assert.ok(Buffer.byteLength(serialized, "utf8") <= 24 * 1_024 * 1_024);
  } catch (error) {
    assert.equal(error instanceof DidError, true);
    assert.ok([
      "did.prism.invalid_operation",
      "did.prism.invalid_wire",
    ].includes(error.code));
    assert.equal(error.stack, undefined);
  }
}

function owned(fields) {
  let freeCount = 0;
  return {
    raw: {
      ...fields,
      free() {
        freeCount += 1;
      },
    },
    freeCount: () => freeCount,
  };
}

const projectionCallbacks = Object.freeze({
  did(serialized) {
    return Object.freeze({
      serialized,
      method: "peer",
      methodSpecificId: serialized.slice("did:peer:".length),
    });
  },
  document(serialized) {
    return Object.freeze(JSON.parse(serialized));
  },
});

const prismProjectionCallbacks = Object.freeze({
  did(serialized) {
    return Object.freeze({
      serialized,
      method: "prism",
      methodSpecificId: serialized.slice("did:prism:".length),
    });
  },
  document(serialized) {
    return Object.freeze(JSON.parse(serialized));
  },
});

test("deterministically fuzzes peer adapter ingress and projection boundaries", () => {
  const next = generator(SEED);

  for (let index = 0; index < CASES; index += 1) {
    assertStableIngressOutcome(() => serializePeerDidNumalgo4Input(numalgo4Candidate(next)));
    assertStableIngressOutcome(() => serializePeerDidNumalgo2Input(numalgo2Candidate(next)));
    assertStablePrismIngress(() => serializePrismCreateInput(prismCreateCandidate(next)));
    assertStablePrismIngress(() => serializePrismHistory(prismHistoryCandidate(next)));

    const numalgo4 = owned({
      profile: next() % 5 === 0 ? boundedText(next, 24) : "numalgo4",
      longForm: next() % 7 === 0 ? null : "did:peer:4long:input",
      shortForm: next() % 11 === 0 ? 42 : "did:peer:4short",
      inputDocument: next() % 13 === 0 ? "{" : JSON.stringify({ case: index }),
      resolvedDocument: JSON.stringify({ id: "did:peer:4long:input" }),
    });
    try {
      const projected = projectPeerDidNumalgo4(numalgo4.raw, projectionCallbacks);
      assert.equal(Object.isFrozen(projected), true);
      assert.equal(Object.isFrozen(projected.inputDocument), true);
    } catch (error) {
      assert.equal(error?.name, "DidBindingError");
      assert.equal(error?.stack, undefined);
    }
    assert.equal(numalgo4.freeCount(), 1);

    const numalgo2 = owned({
      profile: next() % 5 === 0 ? boundedText(next, 24) : "numalgo2-compatibility",
      did: next() % 7 === 0 ? undefined : "did:peer:2.Vz6Mk",
      resolvedDocument: JSON.stringify({ id: "did:peer:2.Vz6Mk" }),
    });
    try {
      const projected = projectPeerDidNumalgo2Compatibility(
        numalgo2.raw,
        projectionCallbacks,
      );
      assert.equal(Object.isFrozen(projected), true);
    } catch (error) {
      assert.equal(error?.name, "DidBindingError");
      assert.equal(error?.stack, undefined);
    }
    assert.equal(numalgo2.freeCount(), 1);

    const prismDid = owned({
      requestedDid: next() % 7 === 0 ? null : "did:prism:requested",
      canonicalDid: next() % 11 === 0 ? 42 : "did:prism:canonical",
      form: next() % 5 === 0 ? boundedText(next, 16) : "canonical",
      resolvedDocument: next() % 13 === 0
        ? "{"
        : JSON.stringify({ id: "did:prism:requested" }),
    });
    try {
      const projected = projectPrismDid(prismDid.raw, prismProjectionCallbacks);
      assert.equal(Object.isFrozen(projected), true);
    } catch (error) {
      assert.equal(error?.name, "DidBindingError");
      assert.equal(error?.stack, undefined);
    }
    assert.equal(prismDid.freeCount(), 1);

    const prismState = owned({
      requestedDid: "did:prism:requested",
      canonicalDid: next() % 11 === 0 ? undefined : "did:prism:canonical",
      deactivated: next() % 3 === 0 ? true : next() % 5 === 0 ? "false" : false,
      resolvedDocument: next() % 3 === 0
        ? undefined
        : next() % 13 === 0
          ? undefined
          : JSON.stringify({ id: "did:prism:requested" }),
    });
    try {
      const projected = projectPrismDidState(prismState.raw, prismProjectionCallbacks);
      assert.equal(Object.isFrozen(projected), true);
    } catch (error) {
      assert.equal(error?.name, "DidBindingError");
      assert.equal(error?.stack, undefined);
    }
    assert.equal(prismState.freeCount(), 1);
  }
});
