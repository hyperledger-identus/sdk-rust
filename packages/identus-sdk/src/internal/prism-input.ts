import {
  DidError,
  type DidDocumentExtensionValue,
  type PrismDidCreateInput,
  type PrismDidKeyCurve,
  type PrismDidKeyUsage,
} from "../public.js";

const MAX_INPUT_BYTES = 64 * 1_024;
const MAX_DID_BYTES = 128 * 1_024;
const MAX_HISTORY_OPERATIONS = 4_096;
const MAX_SIGNED_OPERATION_BYTES = 128 * 1_024;
const MAX_HISTORY_BYTES = 24 * 1_024 * 1_024;
const MAX_DEPTH = 32;
const MAX_NODES = 8_192;
const MAX_MEMBERS = 256;
const FORBIDDEN_KEYS = new Set(["__proto__", "constructor", "prototype"]);
const USAGES: ReadonlySet<PrismDidKeyUsage> = new Set([
  "master",
  "issuing",
  "keyAgreement",
  "authentication",
  "revocation",
  "capabilityInvocation",
  "capabilityDelegation",
  "vdr",
]);
const CURVES: ReadonlySet<PrismDidKeyCurve> = new Set([
  "secp256k1",
  "Ed25519",
  "X25519",
]);
const BASE64URL = /^[A-Za-z0-9_-]+$/u;

type ClosedObject = Readonly<Record<string, unknown>>;

function invalidOperation(): DidError {
  return new DidError("did.prism.invalid_operation");
}

function invalidWire(): DidError {
  return new DidError("did.prism.invalid_wire");
}

function requireObject(value: unknown): ClosedObject {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw invalidOperation();
  }
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw invalidOperation();
  }
  const ownKeys = Reflect.ownKeys(value);
  if (ownKeys.length > MAX_MEMBERS) {
    throw invalidOperation();
  }
  for (const key of ownKeys) {
    if (typeof key !== "string" || FORBIDDEN_KEYS.has(key)) {
      throw invalidOperation();
    }
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (
      descriptor === undefined
      || !Object.hasOwn(descriptor, "value")
      || descriptor.enumerable !== true
    ) {
      throw invalidOperation();
    }
  }
  return value as ClosedObject;
}

function requireExact(
  value: unknown,
  allowed: ReadonlySet<string>,
  required: ReadonlySet<string>,
): ClosedObject {
  const object = requireObject(value);
  const keys = Object.keys(object);
  if (
    keys.some((key) => !allowed.has(key))
    || [...required].some((key) => !Object.hasOwn(object, key))
  ) {
    throw invalidOperation();
  }
  return object;
}

function requireArray(value: unknown, limit: number): readonly unknown[] {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype) {
    throw invalidOperation();
  }
  if (value.length > limit) {
    throw invalidOperation();
  }
  const ownKeys = Reflect.ownKeys(value);
  if (ownKeys.length !== value.length + 1 || ownKeys.at(-1) !== "length") {
    throw invalidOperation();
  }
  for (let index = 0; index < value.length; index += 1) {
    if (ownKeys[index] !== String(index)) {
      throw invalidOperation();
    }
    const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
    if (
      descriptor === undefined
      || !Object.hasOwn(descriptor, "value")
      || descriptor.enumerable !== true
    ) {
      throw invalidOperation();
    }
  }
  return value;
}

function requireString(value: unknown): string {
  if (typeof value !== "string" || value.length === 0) {
    throw invalidOperation();
  }
  return value;
}

function utf8Bytes(value: string): number {
  return new TextEncoder().encode(value).byteLength;
}

function validateJsonValue(root: unknown): void {
  const pending: { readonly value: unknown; readonly depth: number }[] = [{
    value: root,
    depth: 0,
  }];
  const seen = new WeakSet<object>();
  let nodes = 0;
  while (pending.length > 0) {
    const item = pending.pop();
    if (item === undefined || item.depth > MAX_DEPTH || ++nodes > MAX_NODES) {
      throw invalidOperation();
    }
    const value = item.value;
    if (
      value === null
      || typeof value === "boolean"
      || typeof value === "string"
      || (
        typeof value === "number"
        && Number.isFinite(value)
        && (!Number.isInteger(value) || Number.isSafeInteger(value))
      )
    ) {
      continue;
    }
    if (typeof value !== "object") {
      throw invalidOperation();
    }
    if (seen.has(value)) {
      throw invalidOperation();
    }
    seen.add(value);
    const children = Array.isArray(value)
      ? requireArray(value, MAX_MEMBERS)
      : Object.values(requireObject(value));
    for (const child of children) {
      pending.push({ value: child, depth: item.depth + 1 });
    }
  }
}

export function requirePrismDidInput(value: string): string {
  if (typeof value !== "string") {
    throw new TypeError("PRISM DID input must be a string");
  }
  if (utf8Bytes(value) > MAX_DID_BYTES) {
    throw new DidError("did.prism.invalid_syntax");
  }
  return value;
}

export function serializePrismCreateInput(input: PrismDidCreateInput): string {
  const object = requireExact(
    input,
    new Set(["keys", "services", "contexts"]),
    new Set(["keys"]),
  );
  for (const candidate of requireArray(object.keys, 64)) {
    const key = requireExact(
      candidate,
      new Set(["id", "usage", "curve", "publicKey"]),
      new Set(["id", "usage", "curve", "publicKey"]),
    );
    requireString(key.id);
    if (!USAGES.has(key.usage as PrismDidKeyUsage)) {
      throw invalidOperation();
    }
    if (!CURVES.has(key.curve as PrismDidKeyCurve)) {
      throw invalidOperation();
    }
    const publicKey = requireString(key.publicKey);
    if (!BASE64URL.test(publicKey) || publicKey.includes("=")) {
      throw invalidOperation();
    }
  }
  if (object.services !== undefined) {
    for (const candidate of requireArray(object.services, 64)) {
      const service = requireExact(
        candidate,
        new Set(["id", "type", "serviceEndpoint"]),
        new Set(["id", "type", "serviceEndpoint"]),
      );
      requireString(service.id);
      if (typeof service.type === "string") {
        requireString(service.type);
      } else {
        for (const type of requireArray(service.type, 64)) {
          requireString(type);
        }
      }
      validateJsonValue(service.serviceEndpoint as DidDocumentExtensionValue);
    }
  }
  if (object.contexts !== undefined) {
    for (const context of requireArray(object.contexts, 64)) {
      requireString(context);
    }
  }
  validateJsonValue(input);
  let serialized: string;
  try {
    serialized = JSON.stringify(input);
  } catch {
    throw invalidOperation();
  }
  if (utf8Bytes(serialized) > MAX_INPUT_BYTES) {
    throw invalidOperation();
  }
  return serialized;
}

export function serializePrismHistory(value: readonly string[]): string {
  const operations = requireArray(value, MAX_HISTORY_OPERATIONS);
  let total = 0;
  const copied: string[] = [];
  for (const operation of operations) {
    if (
      typeof operation !== "string"
      || !BASE64URL.test(operation)
      || operation.length % 4 === 1
    ) {
      throw invalidWire();
    }
    const bytes = Math.floor(operation.length * 3 / 4);
    if (bytes > MAX_SIGNED_OPERATION_BYTES) {
      throw invalidWire();
    }
    total += operation.length;
    if (total > MAX_HISTORY_BYTES) {
      throw invalidWire();
    }
    copied.push(operation);
  }
  return JSON.stringify({ operations: copied });
}
