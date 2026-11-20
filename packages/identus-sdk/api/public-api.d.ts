export type DidErrorCode = "did.invalid_did" | "did.invalid_did_url" | "did.invalid_document" | "did.invalid_resolution_options" | "did.invalid_dereferencing_options" | "did.invalid_resolution_result" | "did.invalid_dereferencing_result" | "did.resolution_host_failure" | "did.dereferencing_host_failure" | "did.resolution_provider_unavailable" | "did.dereferencing_provider_unavailable" | "did.operation_aborted" | "did.observer_failure" | "did.peer.wrong_profile" | "did.peer.unsupported_algorithm" | "did.peer.malformed_encoding" | "did.peer.unsupported_codec" | "did.peer.unsupported_purpose" | "did.peer.invalid_input_document" | "did.peer.hash_mismatch" | "did.peer.long_form_required" | "did.peer.resource_limit" | "did.peer.projection_failure" | "did.prism.invalid_syntax" | "did.prism.invalid_wire" | "did.prism.invalid_operation" | "did.prism.invalid_state" | "did.prism.history_provider_unavailable" | "did.prism.history_host_failure" | "adapter.binding_contract" | "adapter.unsupported_lossy_mapping";
export type CryptoErrorCode = "crypto.handle_disposed" | "crypto.invalid_private_key" | "crypto.invalid_public_key" | "crypto.invalid_signature" | "crypto.message_too_large" | "crypto.non_contributory_peer" | "crypto.entropy_unavailable" | "crypto.observer_failure" | "adapter.binding_contract";
export type KeyOperation = "ed25519.generate" | "ed25519.import" | "ed25519.public-key" | "ed25519.sign" | "ed25519.verify" | "ed25519.dispose" | "x25519.generate" | "x25519.import" | "x25519.public-key" | "x25519.derive-shared-secret" | "x25519.dispose";
export interface KeyOperationObservation {
    readonly operation: KeyOperation;
    readonly curve: "Ed25519" | "X25519";
    readonly result: "success" | "error";
    readonly errorCode: CryptoErrorCode | undefined;
    readonly durationMs: number;
}
export type KeyOperationObserver = (observation: KeyOperationObservation) => void;
export interface KeyOperationsProviderOptions {
    readonly observer?: KeyOperationObserver;
}
export interface Ed25519PublicKeyValue {
    readonly algorithm: "Ed25519";
    readonly bytes: Uint8Array;
}
export interface X25519PublicKeyValue {
    readonly algorithm: "X25519";
    readonly bytes: Uint8Array;
}
export interface Ed25519PrivateKeyHandle {
    publicKey(): Ed25519PublicKeyValue;
    sign(message: Uint8Array): Uint8Array;
    dispose(): void;
}
export interface X25519PrivateKeyHandle {
    publicKey(): X25519PublicKeyValue;
    deriveSharedSecret(peerPublicKey: Uint8Array): Uint8Array;
    dispose(): void;
}
export interface KeyOperationsProvider {
    generateEd25519(): Ed25519PrivateKeyHandle;
    importEd25519(privateKey: Uint8Array): Ed25519PrivateKeyHandle;
    verifyEd25519(publicKey: Uint8Array, message: Uint8Array, signature: Uint8Array): boolean;
    generateX25519(): X25519PrivateKeyHandle;
    importX25519(privateKey: Uint8Array): X25519PrivateKeyHandle;
}
export interface InjectedKeyOperationsProviderSelection<Provider extends KeyOperationsProvider = KeyOperationsProvider> {
    readonly kind: "injected";
    readonly provider: Provider;
}
export interface WasmKeyOperationsProviderSelection {
    readonly kind: "wasm";
    readonly options?: KeyOperationsProviderOptions;
}
export type KeyOperationsProviderSelection = InjectedKeyOperationsProviderSelection | WasmKeyOperationsProviderSelection;
export declare class CryptoError extends Error {
    readonly code: CryptoErrorCode;
    constructor(code: CryptoErrorCode);
}
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
export type DidDocumentExtensionValue = null | boolean | number | string | readonly DidDocumentExtensionValue[] | DidDocumentExtensionMapValue;
export interface DidDocumentExtensionMapValue {
    readonly [property: string]: DidDocumentExtensionValue;
}
export type DidDocumentContextEntryValue = string | DidDocumentExtensionMapValue;
export type DidDocumentContextValue = DidDocumentContextEntryValue | readonly DidDocumentContextEntryValue[];
export type DidDocumentControllerValue = DidValue | readonly DidValue[];
export interface DidVerificationMethodValue {
    readonly id: string;
    readonly type: string;
    readonly controller: DidValue;
    readonly properties: DidDocumentExtensionMapValue;
}
export type DidVerificationRelationshipValue = Readonly<{
    kind: "reference";
    reference: string;
}> | Readonly<{
    kind: "embedded";
    method: DidVerificationMethodValue;
}>;
export type DidServiceEndpointEntryValue = string | DidDocumentExtensionMapValue;
export type DidServiceEndpointValue = DidServiceEndpointEntryValue | readonly DidServiceEndpointEntryValue[];
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
export type PeerDidNumalgo4InputDocument = DidDocumentExtensionMapValue;
export type PeerDidNumalgo2Purpose = "verification" | "encryption" | "assertion" | "capabilityInvocation" | "capabilityDelegation";
export interface PeerDidNumalgo2PublicKeyInput {
    readonly purpose: PeerDidNumalgo2Purpose;
    readonly multibase: string;
}
export interface PeerDidNumalgo2ServiceEndpointMapInput {
    readonly uri: string;
    readonly accept?: readonly string[];
    readonly routingKeys?: readonly string[];
}
export type PeerDidNumalgo2ServiceEndpointInput = string | PeerDidNumalgo2ServiceEndpointMapInput | readonly PeerDidNumalgo2ServiceEndpointMapInput[];
export interface PeerDidNumalgo2ServiceInput {
    readonly type: string;
    readonly endpoint: PeerDidNumalgo2ServiceEndpointInput;
    readonly id?: string;
}
export interface PeerDidNumalgo2CompatibilityInput {
    readonly keys: readonly PeerDidNumalgo2PublicKeyInput[];
    readonly services?: readonly PeerDidNumalgo2ServiceInput[];
}
export interface PeerDidNumalgo4Value {
    readonly profile: "numalgo4";
    readonly longForm: DidValue;
    readonly shortForm: DidValue;
    readonly inputDocument: PeerDidNumalgo4InputDocument;
    readonly document: DidDocumentValue;
}
export interface PeerDidNumalgo4ShortValue {
    readonly profile: "numalgo4";
    readonly shortForm: DidValue;
}
export interface PeerDidNumalgo2CompatibilityValue {
    readonly profile: "numalgo2-compatibility";
    readonly did: DidValue;
    readonly document: DidDocumentValue;
}
export type PrismDidKeyUsage = "master" | "issuing" | "keyAgreement" | "authentication" | "revocation" | "capabilityInvocation" | "capabilityDelegation" | "vdr";
export type PrismDidKeyCurve = "secp256k1" | "Ed25519" | "X25519";
export interface PrismDidPublicKeyInput {
    readonly id: string;
    readonly usage: PrismDidKeyUsage;
    readonly curve: PrismDidKeyCurve;
    /** Unpadded base64url public-key bytes. */
    readonly publicKey: string;
}
export interface PrismDidServiceInput {
    readonly id: string;
    readonly type: string | readonly string[];
    readonly serviceEndpoint: DidServiceEndpointValue;
}
export interface PrismDidCreateInput {
    readonly keys: readonly PrismDidPublicKeyInput[];
    readonly services?: readonly PrismDidServiceInput[];
    readonly contexts?: readonly string[];
}
export interface PrismDidValue {
    readonly form: "canonical" | "long-form";
    readonly did: DidValue;
    readonly canonicalDid: DidValue;
    readonly document: DidDocumentValue | undefined;
}
export type PrismDidStateValue = Readonly<{
    status: "active";
    did: DidValue;
    canonicalDid: DidValue;
    document: DidDocumentValue;
}> | Readonly<{
    status: "deactivated";
    did: DidValue;
    canonicalDid: DidValue;
    document: undefined;
}>;
export interface PrismDidHistoryRequestValue {
    readonly did: DidValue;
    readonly canonicalDid: DidValue;
    readonly signal: AbortSignal | undefined;
}
export interface PrismDidHistoryEffect {
    history(request: PrismDidHistoryRequestValue): Promise<readonly string[]>;
}
export interface PrismDidEffects {
    readonly history?: PrismDidHistoryEffect;
}
export type DidResolutionStandardErrorKind = "invalidDid" | "invalidDidDocument" | "notFound" | "representationNotSupported" | "invalidDidUrl" | "methodNotSupported" | "invalidOptions" | "internalError" | "featureNotSupported";
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
export type DidResolutionResultValue = (DidResolutionResultBaseValue & Readonly<{
    kind: "success";
    document: DidDocumentValue;
}>) | (DidResolutionResultBaseValue & Readonly<{
    kind: "failure";
    document: null;
}>) | (DidResolutionResultBaseValue & Readonly<{
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
export type DidUrlDereferencingResultValue = (DidUrlDereferencingResultBaseValue & Readonly<{
    kind: "success";
    content: DidDocumentExtensionValue;
}>) | (DidUrlDereferencingResultBaseValue & Readonly<{
    kind: "failure";
    content: null;
}>);
export type DidOperation = "did.parse" | "did-url.parse" | "did-document.parse" | "did.resolve" | "did-url.dereference" | "did.peer.numalgo4.create" | "did.peer.numalgo4.parse-long" | "did.peer.numalgo4.parse-short" | "did.peer.numalgo4.resolve-short" | "did.peer.numalgo2-compatibility.create" | "did.peer.numalgo2-compatibility.parse" | "did.prism.create" | "did.prism.parse" | "did.prism.resolve-long-form" | "did.prism.resolve-history";
export type DidProviderRoute = "browser-wasm" | "node-wasm" | "legacy";
export type DidMappingId = "did.value.typescript.legacy-v1" | "did-url.value.typescript.legacy-v1" | "did.document.typescript.legacy-v1" | "did.resolution.typescript.host-v1" | "did.peer.typescript.numalgo4-v1" | "did.peer.typescript.numalgo2-compat-v1" | "did.prism.typescript.v1" | "did.error.invalid-did.typescript.legacy-v1" | "did.error.invalid-did-url.typescript.legacy-v1";
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
/** Explicit peer DID capability layered over the stable core provider seam. */
export interface PeerDidProvider extends DidProvider {
    createPeerDidNumalgo4(input: PeerDidNumalgo4InputDocument): PeerDidNumalgo4Value;
    parsePeerDidNumalgo4Long(value: string): PeerDidNumalgo4Value;
    parsePeerDidNumalgo4Short(value: string): PeerDidNumalgo4ShortValue;
    resolvePeerDidNumalgo4Short(shortForm: string, longForm: string): PeerDidNumalgo4Value;
    createPeerDidNumalgo2Compatibility(input: PeerDidNumalgo2CompatibilityInput): PeerDidNumalgo2CompatibilityValue;
    parsePeerDidNumalgo2Compatibility(value: string): PeerDidNumalgo2CompatibilityValue;
}
/** Explicit PRISM DID capability layered over peer and core DID values. */
export interface PrismDidProvider extends PeerDidProvider {
    createPrismDid(input: PrismDidCreateInput): PrismDidValue;
    parsePrismDid(value: string): PrismDidValue;
    resolvePrismDidLongForm(value: string): PrismDidValue;
}
export interface InjectedDidProviderSelection<Provider extends DidProvider = DidProvider> {
    readonly kind: "injected";
    readonly provider: Provider;
}
export interface PeerWasmDidProviderSelection {
    readonly kind: "peer-wasm";
    readonly options?: DidProviderOptions;
}
/** A provider choice made once, before any selected WASM initialization. */
export type DidProviderSelection = InjectedDidProviderSelection | PeerWasmDidProviderSelection;
export interface DidResolutionProvider extends DidProvider {
    resolveDid(did: string, options?: DidResolutionOptions, context?: DidOperationContext): Promise<DidResolutionResultValue>;
}
export interface DidUrlDereferencingProvider extends DidProvider {
    dereferenceDidUrl(didUrl: string, options?: DidDereferencingOptions, context?: DidOperationContext): Promise<DidUrlDereferencingResultValue>;
}
export interface DidQueryProvider extends DidResolutionProvider, DidUrlDereferencingProvider {
}
export interface PeerDidQueryProvider extends DidQueryProvider, PeerDidProvider {
}
export interface PrismDidQueryProvider extends DidQueryProvider, PrismDidProvider {
    resolvePrismDid(did: string, context?: DidOperationContext): Promise<PrismDidStateValue>;
}
export type DidProviderFactory = (options?: DidProviderOptions) => Promise<DidProvider>;
export type PeerDidProviderFactory = (options?: DidProviderOptions) => Promise<PeerDidProvider>;
export type DidQueryProviderFactory = (effects: DidQueryEffects, options?: DidProviderOptions) => Promise<DidQueryProvider>;
export type PeerDidQueryProviderFactory = (effects: DidQueryEffects, options?: DidProviderOptions) => Promise<PeerDidQueryProvider>;
export type PrismDidProviderFactory = (options?: DidProviderOptions) => Promise<PrismDidProvider>;
export type PrismDidQueryProviderFactory = (effects: DidQueryEffects & PrismDidEffects, options?: DidProviderOptions) => Promise<PrismDidQueryProvider>;
export declare class DidError extends Error {
    readonly code: DidErrorCode;
    constructor(code: DidErrorCode);
}
export {};
