import {
  DidError,
  type DidDocumentContextEntryValue,
  type DidDocumentContextValue,
  type DidDocumentControllerValue,
  type DidDocumentExtensionMapValue,
  type DidDocumentExtensionValue,
  type DidDocumentValue,
  type DidServiceEndpointEntryValue,
  type DidServiceEndpointValue,
  type DidServiceValue,
  type DidValue,
  type DidVerificationMethodValue,
  type DidVerificationRelationshipValue,
} from "../public.js";

const MAX_PROJECTED_DEPTH = 64;
const MAX_PROJECTED_NODES = 16_384;
const MAX_CONTAINER_MEMBERS = 128;
const MAX_SERIALIZED_CODE_UNITS = 1_572_864;

const DOCUMENT_FIELDS = new Set([
  "@context",
  "id",
  "controller",
  "alsoKnownAs",
  "verificationMethod",
  "authentication",
  "assertionMethod",
  "keyAgreement",
  "capabilityInvocation",
  "capabilityDelegation",
  "service",
]);
const VERIFICATION_METHOD_FIELDS = new Set(["id", "type", "controller"]);
const SERVICE_FIELDS = new Set(["id", "type", "serviceEndpoint"]);

type MutableJsonContainer = unknown[] | Record<string, unknown>;

interface ProjectionFrame {
  readonly input: readonly unknown[] | Readonly<Record<string, unknown>>;
  readonly output: MutableJsonContainer;
  readonly depth: number;
}

export function bindingFailure(): Error {
  const error = new Error("DID binding contract failed");
  error.name = "DidBindingError";
  Object.defineProperty(error, "stack", {
    configurable: false,
    enumerable: false,
    value: undefined,
    writable: false,
  });
  return Object.freeze(error);
}

function isObject(value: unknown): value is Readonly<Record<string, unknown>> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function defineValue(
  target: MutableJsonContainer,
  key: string | number,
  value: unknown,
): void {
  if (Array.isArray(target)) {
    if (typeof key !== "number") {
      throw bindingFailure();
    }
    target[key] = value;
    return;
  }
  Object.defineProperty(target, key, {
    configurable: true,
    enumerable: true,
    value,
    writable: true,
  });
}

function primitiveValue(value: unknown): DidDocumentExtensionValue | undefined {
  if (value === null || typeof value === "boolean" || typeof value === "string") {
    return value;
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw bindingFailure();
    }
    if (Number.isInteger(value) && !Number.isSafeInteger(value)) {
      throw new DidError("adapter.unsupported_lossy_mapping");
    }
    return value;
  }
  return undefined;
}

function emptyContainer(value: unknown): MutableJsonContainer {
  if (Array.isArray(value)) {
    return [];
  }
  if (isObject(value)) {
    return {};
  }
  throw bindingFailure();
}

export function cloneFrozenJson(value: unknown): DidDocumentExtensionValue {
  const primitive = primitiveValue(value);
  if (primitive !== undefined || value === null) {
    return primitive ?? null;
  }

  const root = emptyContainer(value);
  const frames: ProjectionFrame[] = [{
    input: value as readonly unknown[] | Readonly<Record<string, unknown>>,
    output: root,
    depth: 0,
  }];
  const containers: MutableJsonContainer[] = [root];
  let nodes = 1;

  while (frames.length > 0) {
    const frame = frames.pop();
    if (frame === undefined || frame.depth > MAX_PROJECTED_DEPTH) {
      throw bindingFailure();
    }

    const size = Array.isArray(frame.input)
      ? frame.input.length
      : Object.keys(frame.input).length;
    if (size > MAX_CONTAINER_MEMBERS) {
      throw bindingFailure();
    }
    const entries: readonly (readonly [string | number, unknown])[] =
      Array.isArray(frame.input)
        ? frame.input.map((entry, index) => [index, entry] as const)
        : Object.entries(frame.input);

    for (const [key, child] of entries) {
      nodes += 1;
      if (nodes > MAX_PROJECTED_NODES) {
        throw bindingFailure();
      }
      const childPrimitive = primitiveValue(child);
      if (childPrimitive !== undefined || child === null) {
        defineValue(frame.output, key, childPrimitive ?? null);
        continue;
      }
      if (frame.depth >= MAX_PROJECTED_DEPTH) {
        throw bindingFailure();
      }
      const childOutput = emptyContainer(child);
      defineValue(frame.output, key, childOutput);
      containers.push(childOutput);
      frames.push({
        input: child as readonly unknown[] | Readonly<Record<string, unknown>>,
        output: childOutput,
        depth: frame.depth + 1,
      });
    }
  }

  for (let index = containers.length - 1; index >= 0; index -= 1) {
    Object.freeze(containers[index]);
  }
  return root as DidDocumentExtensionValue;
}

export function requireMap(value: DidDocumentExtensionValue): DidDocumentExtensionMapValue {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw bindingFailure();
  }
  return value as DidDocumentExtensionMapValue;
}

export function requireString(value: DidDocumentExtensionValue | undefined): string {
  if (typeof value !== "string") {
    throw bindingFailure();
  }
  return value;
}

export function requireArray(
  value: DidDocumentExtensionValue | undefined,
): readonly DidDocumentExtensionValue[] {
  if (!Array.isArray(value)) {
    throw bindingFailure();
  }
  return value;
}

export function requireNonEmptyArray(
  value: DidDocumentExtensionValue | undefined,
): readonly DidDocumentExtensionValue[] {
  const values = requireArray(value);
  if (values.length === 0) {
    throw bindingFailure();
  }
  return values;
}

export function has(map: DidDocumentExtensionMapValue, property: string): boolean {
  return Object.hasOwn(map, property);
}

export function optional(
  map: DidDocumentExtensionMapValue,
  property: string,
): DidDocumentExtensionValue | undefined {
  return has(map, property) ? map[property] : undefined;
}

export function didValue(serialized: string): DidValue {
  if (!serialized.startsWith("did:")) {
    throw bindingFailure();
  }
  const methodEnd = serialized.indexOf(":", 4);
  if (methodEnd <= 4 || methodEnd === serialized.length - 1) {
    throw bindingFailure();
  }
  return Object.freeze({
    serialized,
    method: serialized.slice(4, methodEnd),
    methodSpecificId: serialized.slice(methodEnd + 1),
  });
}

export function extensionProperties(
  map: DidDocumentExtensionMapValue,
  reserved: ReadonlySet<string>,
): DidDocumentExtensionMapValue {
  const properties: Record<string, DidDocumentExtensionValue> = {};
  for (const key of Object.keys(map)) {
    if (!reserved.has(key)) {
      Object.defineProperty(properties, key, {
        configurable: true,
        enumerable: true,
        value: map[key],
        writable: true,
      });
    }
  }
  return Object.freeze(properties);
}

function stringArray(value: DidDocumentExtensionValue | undefined): readonly string[] {
  return Object.freeze(requireNonEmptyArray(value).map((entry) => requireString(entry)));
}

function contextValue(
  value: DidDocumentExtensionValue | undefined,
): DidDocumentContextValue | undefined {
  if (value === undefined) {
    return undefined;
  }
  const entry = (item: DidDocumentExtensionValue): DidDocumentContextEntryValue =>
    typeof item === "string" ? item : requireMap(item);
  if (Array.isArray(value)) {
    if (value.length === 0) {
      throw bindingFailure();
    }
    return Object.freeze(value.map(entry));
  }
  return entry(value);
}

function controllerValue(
  value: DidDocumentExtensionValue | undefined,
): DidDocumentControllerValue | undefined {
  if (value === undefined) {
    return undefined;
  }
  if (Array.isArray(value)) {
    if (value.length === 0) {
      throw bindingFailure();
    }
    return Object.freeze(value.map((entry) => didValue(requireString(entry))));
  }
  return didValue(requireString(value));
}

function verificationMethod(
  value: DidDocumentExtensionValue,
): DidVerificationMethodValue {
  const map = requireMap(value);
  const properties = extensionProperties(map, VERIFICATION_METHOD_FIELDS);
  if (Object.keys(properties).length === 0) {
    throw bindingFailure();
  }
  return Object.freeze({
    id: requireString(map.id),
    type: requireString(map.type),
    controller: didValue(requireString(map.controller)),
    properties,
  });
}

function verificationMethods(
  value: DidDocumentExtensionValue | undefined,
): readonly DidVerificationMethodValue[] | undefined {
  return value === undefined
    ? undefined
    : Object.freeze(requireNonEmptyArray(value).map(verificationMethod));
}

function relationship(
  value: DidDocumentExtensionValue,
): DidVerificationRelationshipValue {
  if (typeof value === "string") {
    return Object.freeze({ kind: "reference", reference: value });
  }
  return Object.freeze({ kind: "embedded", method: verificationMethod(value) });
}

function relationships(
  value: DidDocumentExtensionValue | undefined,
): readonly DidVerificationRelationshipValue[] | undefined {
  return value === undefined
    ? undefined
    : Object.freeze(requireNonEmptyArray(value).map(relationship));
}

function serviceEndpointEntry(
  value: DidDocumentExtensionValue,
): DidServiceEndpointEntryValue {
  return typeof value === "string" ? value : requireMap(value);
}

function serviceEndpoint(
  value: DidDocumentExtensionValue | undefined,
): DidServiceEndpointValue {
  if (Array.isArray(value)) {
    if (value.length === 0) {
      throw bindingFailure();
    }
    return Object.freeze(value.map(serviceEndpointEntry));
  }
  if (value === undefined) {
    throw bindingFailure();
  }
  return serviceEndpointEntry(value);
}

function serviceType(
  value: DidDocumentExtensionValue | undefined,
): string | readonly string[] {
  return Array.isArray(value) ? stringArray(value) : requireString(value);
}

function service(value: DidDocumentExtensionValue): DidServiceValue {
  const map = requireMap(value);
  return Object.freeze({
    id: requireString(map.id),
    type: serviceType(map.type),
    serviceEndpoint: serviceEndpoint(map.serviceEndpoint),
    properties: extensionProperties(map, SERVICE_FIELDS),
  });
}

function services(
  value: DidDocumentExtensionValue | undefined,
): readonly DidServiceValue[] | undefined {
  return value === undefined
    ? undefined
    : Object.freeze(requireNonEmptyArray(value).map(service));
}

export function projectDidDocument(
  serialized: string,
  generatedId: string,
): DidDocumentValue {
  if (serialized.length > MAX_SERIALIZED_CODE_UNITS) {
    throw bindingFailure();
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(serialized) as unknown;
  } catch {
    throw bindingFailure();
  }

  const document = requireMap(cloneFrozenJson(parsed));
  const id = requireString(document.id);
  if (id !== generatedId) {
    throw bindingFailure();
  }

  return Object.freeze({
    serialized,
    id: didValue(id),
    context: contextValue(optional(document, "@context")),
    controller: controllerValue(optional(document, "controller")),
    alsoKnownAs: has(document, "alsoKnownAs")
      ? stringArray(document.alsoKnownAs)
      : undefined,
    verificationMethod: verificationMethods(optional(document, "verificationMethod")),
    authentication: relationships(optional(document, "authentication")),
    assertionMethod: relationships(optional(document, "assertionMethod")),
    keyAgreement: relationships(optional(document, "keyAgreement")),
    capabilityInvocation: relationships(optional(document, "capabilityInvocation")),
    capabilityDelegation: relationships(optional(document, "capabilityDelegation")),
    service: services(optional(document, "service")),
    properties: extensionProperties(document, DOCUMENT_FIELDS),
  });
}
