import type {
  DidDereferencingOptions,
  DidDocumentExtensionMapValue,
  DidResolutionOptions,
} from "../public.js";
import {
  bindingFailure,
  cloneFrozenJson,
  requireMap,
} from "./document-projection.js";

const RESOLUTION_KEYS = new Set([
  "accept",
  "expandRelativeUrls",
  "noCache",
  "versionId",
  "versionTime",
  "properties",
]);
const DEREFERENCING_KEYS = new Set([
  "accept",
  "verificationRelationship",
  "properties",
]);
const FORBIDDEN_PROPERTY_KEYS = new Set([
  "__proto__",
  "constructor",
  "prototype",
]);

function requireInputMap(value: unknown, allowed: ReadonlySet<string>): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw bindingFailure();
  }
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw bindingFailure();
  }
  const input = value as Record<string, unknown>;
  for (const key of Object.keys(input)) {
    if (!allowed.has(key)) {
      throw bindingFailure();
    }
  }
  return input;
}

function define(target: Record<string, unknown>, key: string, value: unknown): void {
  Object.defineProperty(target, key, {
    configurable: true,
    enumerable: true,
    value,
    writable: true,
  });
}

function optionalString(input: Record<string, unknown>, key: string): string | undefined {
  const value = input[key];
  if (value === undefined || typeof value === "string") {
    return value;
  }
  throw bindingFailure();
}

function optionalBoolean(input: Record<string, unknown>, key: string): boolean | undefined {
  const value = input[key];
  if (value === undefined || typeof value === "boolean") {
    return value;
  }
  throw bindingFailure();
}

function properties(input: Record<string, unknown>): DidDocumentExtensionMapValue {
  if (input.properties === undefined) {
    return Object.freeze({});
  }
  return requireMap(cloneFrozenJson(input.properties));
}

function appendProperties(
  output: Record<string, unknown>,
  extensions: DidDocumentExtensionMapValue,
  reserved: ReadonlySet<string>,
): void {
  for (const [key, value] of Object.entries(extensions)) {
    if (reserved.has(key) || FORBIDDEN_PROPERTY_KEYS.has(key)) {
      throw bindingFailure();
    }
    define(output, key, value);
  }
}

function appendOptional(
  output: Record<string, unknown>,
  key: string,
  value: unknown,
): void {
  if (value !== undefined) {
    define(output, key, value);
  }
}

export function serializeResolutionOptions(
  options: DidResolutionOptions | undefined,
): string {
  const input = requireInputMap(options ?? {}, RESOLUTION_KEYS);
  const output: Record<string, unknown> = {};
  appendOptional(output, "accept", optionalString(input, "accept"));
  appendOptional(
    output,
    "expandRelativeUrls",
    optionalBoolean(input, "expandRelativeUrls"),
  );
  appendOptional(output, "noCache", optionalBoolean(input, "noCache"));
  appendOptional(output, "versionId", optionalString(input, "versionId"));
  appendOptional(output, "versionTime", optionalString(input, "versionTime"));
  appendProperties(output, properties(input), RESOLUTION_KEYS);
  return JSON.stringify(output);
}

export function serializeDereferencingOptions(
  options: DidDereferencingOptions | undefined,
): string {
  const input = requireInputMap(options ?? {}, DEREFERENCING_KEYS);
  const output: Record<string, unknown> = {};
  appendOptional(output, "accept", optionalString(input, "accept"));
  appendOptional(
    output,
    "verificationRelationship",
    optionalString(input, "verificationRelationship"),
  );
  appendProperties(output, properties(input), DEREFERENCING_KEYS);
  return JSON.stringify(output);
}
