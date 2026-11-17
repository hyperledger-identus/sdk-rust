import type {
  DidDereferencingOptionsValue,
  DidDocumentExtensionMapValue,
  DidDocumentExtensionValue,
  DidDocumentMetadataValue,
  DidResolutionMetadataValue,
  DidResolutionOptionsValue,
  DidResolutionProblemValue,
  DidResolutionResultValue,
  DidResolutionStandardErrorKind,
  DidUrlDereferencingResultValue,
} from "../public.js";
import {
  bindingFailure,
  cloneFrozenJson,
  didValue,
  extensionProperties,
  has,
  optional,
  projectDidDocument,
  requireArray,
  requireMap,
  requireString,
} from "./document-projection.js";

const MAX_RESULT_CODE_UNITS = 3_145_728;
const RESOLUTION_OPTION_FIELDS = new Set([
  "accept",
  "expandRelativeUrls",
  "noCache",
  "versionId",
  "versionTime",
]);
const DEREFERENCING_OPTION_FIELDS = new Set([
  "accept",
  "verificationRelationship",
]);
const OPERATION_METADATA_FIELDS = new Set(["contentType", "error"]);
const PROBLEM_FIELDS = new Set(["type", "title", "detail", "instance"]);
const DOCUMENT_METADATA_FIELDS = new Set([
  "created",
  "updated",
  "deactivated",
  "nextUpdate",
  "versionId",
  "nextVersionId",
  "equivalentId",
  "canonicalId",
]);
const RESOLUTION_RESULT_FIELDS = new Set([
  "didResolutionMetadata",
  "didDocument",
  "didDocumentMetadata",
]);
const DEREFERENCING_RESULT_FIELDS = new Set([
  "didUrlDereferencingMetadata",
  "content",
  "contentMetadata",
]);

const STANDARD_PROBLEMS: Readonly<Record<string, DidResolutionStandardErrorKind>> =
  Object.freeze({
    "https://www.w3.org/ns/did#INVALID_DID": "invalidDid",
    "https://www.w3.org/ns/did#INVALID_DID_DOCUMENT": "invalidDidDocument",
    "https://www.w3.org/ns/did#NOT_FOUND": "notFound",
    "https://www.w3.org/ns/did#REPRESENTATION_NOT_SUPPORTED":
      "representationNotSupported",
    "https://www.w3.org/ns/did#INVALID_DID_URL": "invalidDidUrl",
    "https://www.w3.org/ns/did#METHOD_NOT_SUPPORTED": "methodNotSupported",
    "https://www.w3.org/ns/did#INVALID_OPTIONS": "invalidOptions",
    "https://www.w3.org/ns/did#INTERNAL_ERROR": "internalError",
    "https://www.w3.org/ns/did#FEATURE_NOT_SUPPORTED": "featureNotSupported",
  });

function parseMap(serialized: string): DidDocumentExtensionMapValue {
  if (typeof serialized !== "string" || serialized.length > MAX_RESULT_CODE_UNITS) {
    throw bindingFailure();
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(serialized) as unknown;
  } catch {
    throw bindingFailure();
  }
  return requireMap(cloneFrozenJson(parsed));
}

function optionalString(
  map: DidDocumentExtensionMapValue,
  property: string,
): string | undefined {
  const value = optional(map, property);
  return value === undefined ? undefined : requireString(value);
}

function optionalBoolean(
  map: DidDocumentExtensionMapValue,
  property: string,
): boolean | undefined {
  const value = optional(map, property);
  if (value === undefined || typeof value === "boolean") {
    return value;
  }
  throw bindingFailure();
}

function requireNull(
  map: DidDocumentExtensionMapValue,
  property: string,
): void {
  if (!has(map, property) || map[property] !== null) {
    throw bindingFailure();
  }
}

function requireValue(
  map: DidDocumentExtensionMapValue,
  property: string,
): DidDocumentExtensionValue {
  const value = optional(map, property);
  if (value === undefined) {
    throw bindingFailure();
  }
  return value;
}

function projectProblem(value: DidDocumentExtensionValue): DidResolutionProblemValue {
  const map = requireMap(value);
  const type = requireString(map.type);
  return Object.freeze({
    type,
    title: optionalString(map, "title"),
    detail: optionalString(map, "detail"),
    instance: optionalString(map, "instance"),
    standardKind: STANDARD_PROBLEMS[type],
    properties: extensionProperties(map, PROBLEM_FIELDS),
  });
}

function projectOperationMetadata(
  value: DidDocumentExtensionValue | undefined,
): DidResolutionMetadataValue {
  if (value === undefined) {
    throw bindingFailure();
  }
  const map = requireMap(value);
  return Object.freeze({
    contentType: optionalString(map, "contentType"),
    error: has(map, "error")
      ? projectProblem(requireValue(map, "error"))
      : undefined,
    properties: extensionProperties(map, OPERATION_METADATA_FIELDS),
  });
}

function projectDidArray(
  value: DidDocumentExtensionValue | undefined,
): readonly ReturnType<typeof didValue>[] | undefined {
  if (value === undefined) {
    return undefined;
  }
  return Object.freeze(requireArray(value).map((entry) => didValue(requireString(entry))));
}

function projectDocumentMetadata(
  value: DidDocumentExtensionValue | undefined,
): DidDocumentMetadataValue {
  if (value === undefined) {
    throw bindingFailure();
  }
  const map = requireMap(value);
  return Object.freeze({
    created: optionalString(map, "created"),
    updated: optionalString(map, "updated"),
    deactivated: optionalBoolean(map, "deactivated"),
    nextUpdate: optionalString(map, "nextUpdate"),
    versionId: optionalString(map, "versionId"),
    nextVersionId: optionalString(map, "nextVersionId"),
    equivalentId: projectDidArray(optional(map, "equivalentId")),
    canonicalId: has(map, "canonicalId")
      ? didValue(requireString(map.canonicalId))
      : undefined,
    properties: extensionProperties(map, DOCUMENT_METADATA_FIELDS),
  });
}

export function projectResolutionOptions(
  serialized: string,
): DidResolutionOptionsValue {
  const map = parseMap(serialized);
  return Object.freeze({
    accept: optionalString(map, "accept"),
    expandRelativeUrls: optionalBoolean(map, "expandRelativeUrls"),
    noCache: optionalBoolean(map, "noCache"),
    versionId: optionalString(map, "versionId"),
    versionTime: optionalString(map, "versionTime"),
    properties: extensionProperties(map, RESOLUTION_OPTION_FIELDS),
  });
}

export function projectDereferencingOptions(
  serialized: string,
): DidDereferencingOptionsValue {
  const map = parseMap(serialized);
  return Object.freeze({
    accept: optionalString(map, "accept"),
    verificationRelationship: optionalString(map, "verificationRelationship"),
    properties: extensionProperties(map, DEREFERENCING_OPTION_FIELDS),
  });
}

export function projectDidResolutionResult(
  serialized: string,
  generatedKind: string,
): DidResolutionResultValue {
  const map = parseMap(serialized);
  if (Object.keys(extensionProperties(map, RESOLUTION_RESULT_FIELDS)).length !== 0) {
    throw bindingFailure();
  }
  const resolutionMetadata = projectOperationMetadata(map.didResolutionMetadata);
  const documentMetadata = projectDocumentMetadata(map.didDocumentMetadata);

  if (generatedKind === "success") {
    const documentMap = requireMap(requireValue(map, "didDocument"));
    const documentSerialized = JSON.stringify(documentMap);
    return Object.freeze({
      kind: "success",
      serialized,
      resolutionMetadata,
      documentMetadata,
      document: projectDidDocument(
        documentSerialized,
        requireString(documentMap.id),
      ),
    });
  }
  requireNull(map, "didDocument");
  if (generatedKind === "error" && resolutionMetadata.error !== undefined) {
    return Object.freeze({
      kind: "failure",
      serialized,
      resolutionMetadata,
      documentMetadata,
      document: null,
    });
  }
  if (
    generatedKind === "deactivated"
    && resolutionMetadata.error === undefined
    && documentMetadata.deactivated === true
  ) {
    return Object.freeze({
      kind: "deactivated",
      serialized,
      resolutionMetadata,
      documentMetadata,
      document: null,
    });
  }
  throw bindingFailure();
}

export function projectDidDereferencingResult(
  serialized: string,
  generatedKind: string,
): DidUrlDereferencingResultValue {
  const map = parseMap(serialized);
  if (Object.keys(extensionProperties(map, DEREFERENCING_RESULT_FIELDS)).length !== 0) {
    throw bindingFailure();
  }
  const dereferencingMetadata = projectOperationMetadata(
    map.didUrlDereferencingMetadata,
  );
  const contentMetadataMap = requireMap(
    requireValue(map, "contentMetadata"),
  );
  const contentMetadata = Object.freeze({ properties: contentMetadataMap });

  if (generatedKind === "success") {
    const content = map.content;
    if (content === null || content === undefined) {
      throw bindingFailure();
    }
    return Object.freeze({
      kind: "success",
      serialized,
      dereferencingMetadata,
      contentMetadata,
      content,
    });
  }
  requireNull(map, "content");
  if (generatedKind === "error" && dereferencingMetadata.error !== undefined) {
    return Object.freeze({
      kind: "failure",
      serialized,
      dereferencingMetadata,
      contentMetadata,
      content: null,
    });
  }
  throw bindingFailure();
}
