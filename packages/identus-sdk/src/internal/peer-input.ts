import {
  DidError,
  type PeerDidNumalgo2CompatibilityInput,
  type PeerDidNumalgo2Purpose,
  type PeerDidNumalgo2ServiceEndpointMapInput,
  type PeerDidNumalgo2ServiceInput,
  type PeerDidNumalgo4InputDocument,
} from "../public.js";
import { cloneFrozenJson, requireMap } from "./document-projection.js";

const MAX_NUMALGO4_INPUT_BYTES = 16 * 1_024;
const MAX_NUMALGO2_INPUT_BYTES = 24 * 1_024;
const MAX_DEPTH = 32;
const MAX_NODES = 4_096;
const MAX_MEMBERS = 128;
const MAX_ELEMENTS = 32;
const FORBIDDEN_KEYS = new Set(["__proto__", "constructor", "prototype"]);
const PURPOSES: ReadonlySet<PeerDidNumalgo2Purpose> = new Set([
  "verification",
  "encryption",
  "assertion",
  "capabilityInvocation",
  "capabilityDelegation",
]);

type ClosedObject = Readonly<Record<string, unknown>>;

function invalidInput(): DidError {
  return new DidError("did.peer.invalid_input_document");
}

function resourceLimit(): DidError {
  return new DidError("did.peer.resource_limit");
}

function requireClosedObject(value: unknown): ClosedObject {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw invalidInput();
  }
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw invalidInput();
  }
  for (const key of Reflect.ownKeys(value)) {
    if (typeof key !== "string" || FORBIDDEN_KEYS.has(key)) {
      throw invalidInput();
    }
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (
      descriptor === undefined
      || !("value" in descriptor)
      || descriptor.enumerable !== true
    ) {
      throw invalidInput();
    }
  }
  return value as ClosedObject;
}

function requireExactObject(
  value: unknown,
  allowed: ReadonlySet<string>,
  required: ReadonlySet<string>,
): ClosedObject {
  const object = requireClosedObject(value);
  const keys = Object.keys(object);
  if (
    keys.some((key) => !allowed.has(key))
    || [...required].some((key) => !Object.hasOwn(object, key))
  ) {
    throw invalidInput();
  }
  return object;
}

function requireArray(value: unknown): readonly unknown[] {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype) {
    throw invalidInput();
  }
  const ownKeys = Reflect.ownKeys(value);
  if (ownKeys.length !== value.length + 1 || ownKeys.at(-1) !== "length") {
    throw invalidInput();
  }
  for (let index = 0; index < value.length; index += 1) {
    if (ownKeys[index] !== String(index)) {
      throw invalidInput();
    }
    const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
    if (
      descriptor === undefined
      || !("value" in descriptor)
      || descriptor.enumerable !== true
    ) {
      throw invalidInput();
    }
  }
  return value;
}

function requireString(value: unknown): string {
  if (typeof value !== "string" || value.length === 0) {
    throw invalidInput();
  }
  return value;
}

function utf8Length(value: string, limit: number): number {
  let bytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const codeUnit = value.charCodeAt(index);
    if (codeUnit <= 0x7f) {
      bytes += 1;
    } else if (codeUnit <= 0x7ff) {
      bytes += 2;
    } else if (codeUnit >= 0xd800 && codeUnit <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (next >= 0xdc00 && next <= 0xdfff) {
        bytes += 4;
        index += 1;
      } else {
        bytes += 3;
      }
    } else {
      bytes += 3;
    }
    if (bytes > limit) {
      throw resourceLimit();
    }
  }
  return bytes;
}

function assertBoundedJsonTree(root: unknown): void {
  const pending: { readonly value: unknown; readonly depth: number }[] = [{
    value: root,
    depth: 0,
  }];
  const seen = new WeakSet<object>();
  let nodes = 0;
  while (pending.length > 0) {
    const frame = pending.pop();
    if (frame === undefined) {
      throw invalidInput();
    }
    nodes += 1;
    if (nodes > MAX_NODES || frame.depth > MAX_DEPTH) {
      throw resourceLimit();
    }
    const value = frame.value;
    if (
      value === null
      || typeof value === "string"
      || typeof value === "boolean"
    ) {
      continue;
    }
    if (typeof value === "number") {
      if (
        !Number.isFinite(value)
        || (Number.isInteger(value) && !Number.isSafeInteger(value))
      ) {
        throw invalidInput();
      }
      continue;
    }
    if (typeof value !== "object" || seen.has(value)) {
      throw invalidInput();
    }
    seen.add(value);
    if (Array.isArray(value)) {
      const array = requireArray(value);
      if (array.length > MAX_MEMBERS) {
        throw resourceLimit();
      }
      for (let index = array.length - 1; index >= 0; index -= 1) {
        pending.push({ value: array[index], depth: frame.depth + 1 });
      }
    } else {
      const object = requireClosedObject(value);
      const entries = Object.entries(object);
      if (entries.length > MAX_MEMBERS) {
        throw resourceLimit();
      }
      for (let index = entries.length - 1; index >= 0; index -= 1) {
        const entry = entries[index];
        if (entry === undefined) {
          throw invalidInput();
        }
        pending.push({ value: entry[1], depth: frame.depth + 1 });
      }
    }
  }
}

export function serializePeerDidNumalgo4Input(
  input: PeerDidNumalgo4InputDocument,
): string {
  assertBoundedJsonTree(input);
  const cloned = requireMap(cloneFrozenJson(input, FORBIDDEN_KEYS));
  if (Object.hasOwn(cloned, "id")) {
    throw invalidInput();
  }
  const serialized = JSON.stringify(cloned);
  utf8Length(serialized, MAX_NUMALGO4_INPUT_BYTES);
  return serialized;
}

function stringArray(value: unknown): readonly string[] {
  const array = requireArray(value);
  if (array.length > MAX_ELEMENTS) {
    throw resourceLimit();
  }
  return array.map(requireString);
}

function copyEndpointMap(value: unknown): PeerDidNumalgo2ServiceEndpointMapInput {
  const object = requireExactObject(
    value,
    new Set(["uri", "accept", "routingKeys"]),
    new Set(["uri"]),
  );
  return Object.freeze({
    uri: requireString(object.uri),
    ...(object.accept === undefined
      ? {}
      : { accept: Object.freeze(stringArray(object.accept)) }),
    ...(object.routingKeys === undefined
      ? {}
      : { routingKeys: Object.freeze(stringArray(object.routingKeys)) }),
  });
}

function copyEndpoint(value: unknown): unknown {
  if (typeof value === "string") {
    return requireString(value);
  }
  if (Array.isArray(value)) {
    const endpoints = requireArray(value);
    if (endpoints.length === 0) {
      throw invalidInput();
    }
    if (endpoints.length > MAX_ELEMENTS) {
      throw resourceLimit();
    }
    return endpoints.map(copyEndpointMap);
  }
  return copyEndpointMap(value);
}

function copyService(value: unknown): PeerDidNumalgo2ServiceInput {
  const object = requireExactObject(
    value,
    new Set(["type", "endpoint", "id"]),
    new Set(["type", "endpoint"]),
  );
  return Object.freeze({
    type: requireString(object.type),
    endpoint: copyEndpoint(object.endpoint) as PeerDidNumalgo2ServiceInput["endpoint"],
    ...(object.id === undefined ? {} : { id: requireString(object.id) }),
  });
}

export function serializePeerDidNumalgo2Input(
  input: PeerDidNumalgo2CompatibilityInput,
): string {
  const object = requireExactObject(
    input,
    new Set(["keys", "services"]),
    new Set(["keys"]),
  );
  const keys = requireArray(object.keys);
  const services = object.services === undefined ? [] : requireArray(object.services);
  if (keys.length === 0) {
    throw invalidInput();
  }
  if (keys.length + services.length > MAX_ELEMENTS) {
    throw resourceLimit();
  }
  const copiedKeys = keys.map((value) => {
    const key = requireExactObject(
      value,
      new Set(["purpose", "multibase"]),
      new Set(["purpose", "multibase"]),
    );
    const purpose = requireString(key.purpose) as PeerDidNumalgo2Purpose;
    if (!PURPOSES.has(purpose)) {
      throw new DidError("did.peer.unsupported_purpose");
    }
    return Object.freeze({
      purpose,
      multibase: requireString(key.multibase),
    });
  });
  const serialized = JSON.stringify({
    keys: copiedKeys,
    services: services.map(copyService),
  });
  utf8Length(serialized, MAX_NUMALGO2_INPUT_BYTES);
  return serialized;
}
