export type DidErrorCode =
  | "did.invalid_did"
  | "did.invalid_did_url"
  | "did.invalid_document"
  | "did.invalid_resolution_options"
  | "did.invalid_dereferencing_options"
  | "did.invalid_resolution_result"
  | "did.invalid_dereferencing_result"
  | "did.resolution_host_failure"
  | "did.dereferencing_host_failure"
  | "did.resolution_provider_unavailable"
  | "did.dereferencing_provider_unavailable"
  | "did.operation_aborted"
  | "did.observer_failure"
  | "adapter.binding_contract"
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

export type DidResolutionStandardErrorKind =
  | "invalidDid"
  | "invalidDidDocument"
  | "notFound"
  | "representationNotSupported"
  | "invalidDidUrl"
  | "methodNotSupported"
  | "invalidOptions"
  | "internalError"
  | "featureNotSupported";

export interface DidResolutionOptions {
  readonly accept?: string;
  readonly expandRelativeUrls?: boolean;
  readonly noCache?: boolean;
  readonly versionId?: string;
  readonly versionTime?: string;
  readonly properties?: DidDocumentExtensionMapValue;
}

export interface DidResolutionOptionsValue {
  readonly accept: string | undefined;
  readonly expandRelativeUrls: boolean | undefined;
  readonly noCache: boolean | undefined;
  readonly versionId: string | undefined;
  readonly versionTime: string | undefined;
  readonly properties: DidDocumentExtensionMapValue;
}

export interface DidDereferencingOptions {
  readonly accept?: string;
  readonly verificationRelationship?: string;
  readonly properties?: DidDocumentExtensionMapValue;
}

export interface DidDereferencingOptionsValue {
  readonly accept: string | undefined;
  readonly verificationRelationship: string | undefined;
  readonly properties: DidDocumentExtensionMapValue;
}

export interface DidOperationContext {
  readonly signal?: AbortSignal;
}

export interface DidResolutionRequestValue {
  readonly did: DidValue;
  readonly options: DidResolutionOptionsValue;
  readonly signal: AbortSignal | undefined;
}

export interface DidDereferencingRequestValue {
  readonly didUrl: DidUrlValue;
  readonly options: DidDereferencingOptionsValue;
  readonly signal: AbortSignal | undefined;
}

export interface DidResolverEffect {
  resolve(request: DidResolutionRequestValue): Promise<string>;
}

export interface DidUrlDereferencerEffect {
  dereference(request: DidDereferencingRequestValue): Promise<string>;
}

export interface DidQueryEffects {
  readonly resolver?: DidResolverEffect;
  readonly dereferencer?: DidUrlDereferencerEffect;
}

export interface DidResolutionProblemValue {
  readonly type: string;
  readonly title: string | undefined;
  readonly detail: string | undefined;
  readonly instance: string | undefined;
  readonly standardKind: DidResolutionStandardErrorKind | undefined;
  readonly properties: DidDocumentExtensionMapValue;
}

export interface DidResolutionMetadataValue {
  readonly contentType: string | undefined;
  readonly error: DidResolutionProblemValue | undefined;
  readonly properties: DidDocumentExtensionMapValue;
}

export interface DidDocumentMetadataValue {
  readonly created: string | undefined;
  readonly updated: string | undefined;
  readonly deactivated: boolean | undefined;
  readonly nextUpdate: string | undefined;
  readonly versionId: string | undefined;
  readonly nextVersionId: string | undefined;
  readonly equivalentId: readonly DidValue[] | undefined;
  readonly canonicalId: DidValue | undefined;
  readonly properties: DidDocumentExtensionMapValue;
}

interface DidResolutionResultBaseValue {
  readonly serialized: string;
  readonly resolutionMetadata: DidResolutionMetadataValue;
  readonly documentMetadata: DidDocumentMetadataValue;
}

export type DidResolutionResultValue =
  | (DidResolutionResultBaseValue & Readonly<{
      kind: "success";
      document: DidDocumentValue;
    }>)
  | (DidResolutionResultBaseValue & Readonly<{
      kind: "failure";
      document: null;
    }>)
  | (DidResolutionResultBaseValue & Readonly<{
      kind: "deactivated";
      document: null;
    }>);

export interface DidUrlContentMetadataValue {
  readonly properties: DidDocumentExtensionMapValue;
}

interface DidUrlDereferencingResultBaseValue {
  readonly serialized: string;
  readonly dereferencingMetadata: DidResolutionMetadataValue;
  readonly contentMetadata: DidUrlContentMetadataValue;
}

export type DidUrlDereferencingResultValue =
  | (DidUrlDereferencingResultBaseValue & Readonly<{
      kind: "success";
      content: DidDocumentExtensionValue;
    }>)
  | (DidUrlDereferencingResultBaseValue & Readonly<{
      kind: "failure";
      content: null;
    }>);

export type DidOperation =
  | "did.parse"
  | "did-url.parse"
  | "did-document.parse"
  | "did.resolve"
  | "did-url.dereference";

export type DidProviderRoute = "browser-wasm" | "node-wasm" | "legacy";

export type DidMappingId =
  | "did.value.typescript.legacy-v1"
  | "did-url.value.typescript.legacy-v1"
  | "did.document.typescript.legacy-v1"
  | "did.resolution.typescript.host-v1"
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

export interface DidResolutionProvider extends DidProvider {
  resolveDid(
    did: string,
    options?: DidResolutionOptions,
    context?: DidOperationContext,
  ): Promise<DidResolutionResultValue>;
}

export interface DidUrlDereferencingProvider extends DidProvider {
  dereferenceDidUrl(
    didUrl: string,
    options?: DidDereferencingOptions,
    context?: DidOperationContext,
  ): Promise<DidUrlDereferencingResultValue>;
}

export interface DidQueryProvider
  extends DidResolutionProvider, DidUrlDereferencingProvider {}

export type DidProviderFactory = (
  options?: DidProviderOptions,
) => Promise<DidProvider>;

export type DidQueryProviderFactory = (
  effects: DidQueryEffects,
  options?: DidProviderOptions,
) => Promise<DidQueryProvider>;

const ERROR_MESSAGES: Readonly<Record<DidErrorCode, string>> = Object.freeze({
  "did.invalid_did": "DID value is invalid",
  "did.invalid_did_url": "DID URL value is invalid",
  "did.invalid_document": "DID document is invalid",
  "did.invalid_resolution_options": "DID resolution options are invalid",
  "did.invalid_dereferencing_options": "DID dereferencing options are invalid",
  "did.invalid_resolution_result": "DID resolution result is invalid",
  "did.invalid_dereferencing_result": "DID dereferencing result is invalid",
  "did.resolution_host_failure": "DID resolution host failed",
  "did.dereferencing_host_failure": "DID dereferencing host failed",
  "did.resolution_provider_unavailable": "DID resolution provider is unavailable",
  "did.dereferencing_provider_unavailable": "DID dereferencing provider is unavailable",
  "did.operation_aborted": "DID operation was aborted",
  "did.observer_failure": "DID observation failed",
  "adapter.binding_contract": "DID binding contract failed",
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
