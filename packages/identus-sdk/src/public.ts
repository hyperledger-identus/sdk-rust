export type DidErrorCode =
  | "did.invalid_did"
  | "did.invalid_did_url"
  | "did.invalid_document"
  | "adapter.unsupported_lossy_mapping";

export interface DidValue {
  readonly serialized: string;
  readonly method: string;
  readonly methodSpecificId: string;
}

export interface DidUrlValue extends DidValue {
  readonly did: string;
  readonly path: string;
  readonly query: string | undefined;
  readonly fragment: string | undefined;
}

export type DidDocumentExtensionValue =
  | null
  | boolean
  | number
  | string
  | readonly DidDocumentExtensionValue[]
  | DidDocumentExtensionMapValue;

export interface DidDocumentExtensionMapValue {
  readonly [property: string]: DidDocumentExtensionValue;
}

export type DidDocumentContextEntryValue =
  | string
  | DidDocumentExtensionMapValue;

export type DidDocumentContextValue =
  | DidDocumentContextEntryValue
  | readonly DidDocumentContextEntryValue[];

export type DidDocumentControllerValue = DidValue | readonly DidValue[];

export interface DidVerificationMethodValue {
  readonly id: string;
  readonly type: string;
  readonly controller: DidValue;
  readonly properties: DidDocumentExtensionMapValue;
}

export type DidVerificationRelationshipValue =
  | Readonly<{
      kind: "reference";
      reference: string;
    }>
  | Readonly<{
      kind: "embedded";
      method: DidVerificationMethodValue;
    }>;

export type DidServiceEndpointEntryValue =
  | string
  | DidDocumentExtensionMapValue;

export type DidServiceEndpointValue =
  | DidServiceEndpointEntryValue
  | readonly DidServiceEndpointEntryValue[];

export interface DidServiceValue {
  readonly id: string;
  readonly type: string | readonly string[];
  readonly serviceEndpoint: DidServiceEndpointValue;
  readonly properties: DidDocumentExtensionMapValue;
}

export interface DidDocumentValue {
  readonly serialized: string;
  readonly id: DidValue;
  readonly context: DidDocumentContextValue | undefined;
  readonly controller: DidDocumentControllerValue | undefined;
  readonly alsoKnownAs: readonly string[] | undefined;
  readonly verificationMethod: readonly DidVerificationMethodValue[] | undefined;
  readonly authentication: readonly DidVerificationRelationshipValue[] | undefined;
  readonly assertionMethod: readonly DidVerificationRelationshipValue[] | undefined;
  readonly keyAgreement: readonly DidVerificationRelationshipValue[] | undefined;
  readonly capabilityInvocation: readonly DidVerificationRelationshipValue[] | undefined;
  readonly capabilityDelegation: readonly DidVerificationRelationshipValue[] | undefined;
  readonly service: readonly DidServiceValue[] | undefined;
  readonly properties: DidDocumentExtensionMapValue;
}

export type DidOperation =
  | "did.parse"
  | "did-url.parse"
  | "did-document.parse";

export type DidProviderRoute = "browser-wasm" | "node-wasm" | "legacy";

export type DidMappingId =
  | "did.value.typescript.legacy-v1"
  | "did-url.value.typescript.legacy-v1"
  | "did.document.typescript.legacy-v1"
  | "did.error.invalid-did.typescript.legacy-v1"
  | "did.error.invalid-did-url.typescript.legacy-v1";

export interface DidObservation {
  readonly operation: DidOperation;
  readonly route: DidProviderRoute;
  readonly result: "success" | "error";
  readonly mappingId: DidMappingId;
  readonly errorCode: DidErrorCode | undefined;
  readonly durationMs: number;
}

export type DidObserver = (observation: DidObservation) => void;

export interface DidProviderOptions {
  readonly observer?: DidObserver;
}

export interface DidProvider {
  parseDid(value: string): DidValue;
  parseDidUrl(value: string): DidUrlValue;
  parseDidDocument(value: string): DidDocumentValue;
}

export type DidProviderFactory = (
  options?: DidProviderOptions,
) => Promise<DidProvider>;

const ERROR_MESSAGES: Readonly<Record<DidErrorCode, string>> = Object.freeze({
  "did.invalid_did": "DID value is invalid",
  "did.invalid_did_url": "DID URL value is invalid",
  "did.invalid_document": "DID document is invalid",
  "adapter.unsupported_lossy_mapping": "DID document cannot be projected safely",
});

export class DidError extends Error {
  readonly code: DidErrorCode;

  constructor(code: DidErrorCode) {
    super(ERROR_MESSAGES[code]);
    this.name = "DidError";
    this.code = code;
    Object.defineProperty(this, "stack", {
      configurable: false,
      enumerable: false,
      value: undefined,
      writable: false,
    });
    Object.freeze(this);
  }
}
