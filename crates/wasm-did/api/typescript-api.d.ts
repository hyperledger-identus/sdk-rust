/* tslint:disable */
/* eslint-disable */

/**
 * Owned normalized request for one DID URL dereferencing host effect.
 */
export class DidDereferencingRequestView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Base DID portion.
     */
    readonly did: string;
    /**
     * Exact validated DID URL.
     */
    readonly didUrl: string;
    /**
     * Fragment without the leading hash.
     */
    readonly fragment: string | undefined;
    /**
     * DID method name.
     */
    readonly method: string;
    /**
     * Method-specific identifier.
     */
    readonly methodSpecificId: string;
    /**
     * Deterministic normalized dereferencing options JSON.
     */
    readonly options: string;
    /**
     * Path including its leading slash when present.
     */
    readonly path: string;
    /**
     * Query without the leading question mark.
     */
    readonly query: string | undefined;
}

/**
 * Owned normalized DID URL dereferencing result.
 */
export class DidDereferencingResultView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Closed result branch: `success` or `error`.
     */
    readonly kind: string;
    /**
     * Deterministic normalized W3C DID URL Dereferencing result JSON.
     */
    readonly serialized: string;
}

/**
 * Owned JavaScript view of a validated DID Document.
 *
 * `serialized` is deterministic normalized JSON produced from the validated
 * Rust model. It preserves JSON cardinality and semantic values, but does not
 * claim original whitespace, object member order, or RFC 8785 canonical JSON.
 */
export class DidDocumentView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Exact validated DID subject.
     */
    readonly id: string;
    /**
     * Deterministic normalized JSON for the validated DID Document.
     */
    readonly serialized: string;
}

/**
 * Closed JavaScript failure carrying only a stable SDK-owned code.
 */
export class DidParseError {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Stable code without caller or implementation text.
     */
    readonly code: string;
}

/**
 * Owned normalized request for one DID resolution host effect.
 */
export class DidResolutionRequestView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Exact validated DID.
     */
    readonly did: string;
    /**
     * DID method name.
     */
    readonly method: string;
    /**
     * Method-specific identifier.
     */
    readonly methodSpecificId: string;
    /**
     * Deterministic normalized resolution options JSON.
     */
    readonly options: string;
}

/**
 * Owned normalized DID resolution result.
 */
export class DidResolutionResultView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Closed result branch: `success`, `error`, or `deactivated`.
     */
    readonly kind: string;
    /**
     * Deterministic normalized W3C DID Resolution result JSON.
     */
    readonly serialized: string;
}

/**
 * Owned JavaScript view of a validated DID URL.
 */
export class DidUrlView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Base DID portion.
     */
    readonly did: string;
    /**
     * Fragment without the leading hash.
     */
    readonly fragment: string | undefined;
    /**
     * DID method name.
     */
    readonly method: string;
    /**
     * Method-specific identifier.
     */
    readonly methodSpecificId: string;
    /**
     * Path including its leading slash when present.
     */
    readonly path: string;
    /**
     * Query without the leading question mark.
     */
    readonly query: string | undefined;
    /**
     * Exact validated DID URL value.
     */
    readonly value: string;
}

/**
 * Owned JavaScript view of a validated DID.
 */
export class DidView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * DID method name.
     */
    readonly method: string;
    /**
     * Method-specific identifier.
     */
    readonly methodSpecificId: string;
    /**
     * Exact validated DID value.
     */
    readonly value: string;
}

/**
 * Owned JavaScript result for explicit numalgo-2 compatibility operations.
 */
export class PeerDidNumalgo2CompatibilityView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Validated compatibility peer DID.
     */
    readonly did: string;
    /**
     * Stable explicit compatibility-profile identity.
     */
    readonly profile: string;
    /**
     * Compact normalized locally resolved DID Document.
     */
    readonly resolvedDocument: string;
}

/**
 * Owned JavaScript result for short-form validation without resolution.
 */
export class PeerDidNumalgo4ShortView {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Stable explicit profile identity.
     */
    readonly profile: string;
    /**
     * Validated short-form peer DID.
     */
    readonly shortForm: string;
}

/**
 * Owned JavaScript result for numalgo-4 creation or local resolution.
 */
export class PeerDidNumalgo4View {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compact normalized public input document without a root `id`.
     */
    readonly inputDocument: string;
    /**
     * Verified long-form peer DID.
     */
    readonly longForm: string;
    /**
     * Stable explicit profile identity.
     */
    readonly profile: string;
    /**
     * Compact normalized locally resolved DID Document.
     */
    readonly resolvedDocument: string;
    /**
     * Derived short-form peer DID.
     */
    readonly shortForm: string;
}

/**
 * Return the binding contract version.
 */
export function bindingApiVersion(): number;

/**
 * Create and locally resolve an explicit numalgo-2 compatibility DID.
 */
export function createPeerDidNumalgo2Compatibility(input_json: string): PeerDidNumalgo2CompatibilityView;

/**
 * Create and locally resolve the current numalgo-4 profile.
 */
export function createPeerDidNumalgo4(input_document_json: string): PeerDidNumalgo4View;

/**
 * Parse a bounded DID into an owned JavaScript view.
 */
export function parseDid(value: string): DidView;

/**
 * Validate and normalize raw DID URL dereferencing result JSON.
 */
export function parseDidDereferencingResult(result_json: string): DidDereferencingResultView;

/**
 * Parse a bounded DID Document into an owned JavaScript view.
 *
 * Rust validates the raw JSON before any TypeScript projection. Documents
 * containing an integer outside JavaScript's exact safe range are valid Rust
 * domain values but cannot cross this typed adapter without corruption, so
 * they fail with an explicit unsupported-mapping code.
 */
export function parseDidDocument(value: string): DidDocumentView;

/**
 * Validate, correlate, and normalize raw DID resolution result JSON.
 */
export function parseDidResolutionResult(did: string, result_json: string): DidResolutionResultView;

/**
 * Parse a bounded DID URL into an owned JavaScript view.
 */
export function parseDidUrl(value: string): DidUrlView;

/**
 * Parse and locally resolve an explicit numalgo-2 compatibility DID.
 */
export function parsePeerDidNumalgo2Compatibility(did: string): PeerDidNumalgo2CompatibilityView;

/**
 * Parse, verify, and locally resolve a numalgo-4 long form.
 */
export function parsePeerDidNumalgo4Long(long_form: string): PeerDidNumalgo4View;

/**
 * Validate a numalgo-4 short form without implying resolvability.
 */
export function parsePeerDidNumalgo4Short(short_form: string): PeerDidNumalgo4ShortView;

/**
 * Validate and normalize one DID URL dereferencing request before host invocation.
 */
export function prepareDidDereferencing(did_url: string, options_json: string): DidDereferencingRequestView;

/**
 * Validate and normalize one DID resolution request before host invocation.
 */
export function prepareDidResolution(did: string, options_json: string): DidResolutionRequestView;

/**
 * Resolve a short form only with matching caller-supplied long-form material.
 */
export function resolvePeerDidNumalgo4Short(short_form: string, long_form: string): PeerDidNumalgo4View;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_peerdidnumalgo2compatibilityview_free: (a: number, b: number) => void;
    readonly __wbg_peerdidnumalgo4shortview_free: (a: number, b: number) => void;
    readonly __wbg_peerdidnumalgo4view_free: (a: number, b: number) => void;
    readonly createPeerDidNumalgo2Compatibility: (a: number, b: number) => [number, number, number];
    readonly createPeerDidNumalgo4: (a: number, b: number) => [number, number, number];
    readonly parsePeerDidNumalgo2Compatibility: (a: number, b: number) => [number, number, number];
    readonly parsePeerDidNumalgo4Long: (a: number, b: number) => [number, number, number];
    readonly parsePeerDidNumalgo4Short: (a: number, b: number) => [number, number, number];
    readonly peerdidnumalgo2compatibilityview_did: (a: number) => [number, number];
    readonly peerdidnumalgo2compatibilityview_profile: (a: number) => [number, number];
    readonly peerdidnumalgo2compatibilityview_resolvedDocument: (a: number) => [number, number];
    readonly peerdidnumalgo4shortview_profile: (a: number) => [number, number];
    readonly peerdidnumalgo4shortview_shortForm: (a: number) => [number, number];
    readonly peerdidnumalgo4view_inputDocument: (a: number) => [number, number];
    readonly peerdidnumalgo4view_longForm: (a: number) => [number, number];
    readonly peerdidnumalgo4view_profile: (a: number) => [number, number];
    readonly peerdidnumalgo4view_resolvedDocument: (a: number) => [number, number];
    readonly peerdidnumalgo4view_shortForm: (a: number) => [number, number];
    readonly resolvePeerDidNumalgo4Short: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly __wbg_diddereferencingrequestview_free: (a: number, b: number) => void;
    readonly __wbg_diddereferencingresultview_free: (a: number, b: number) => void;
    readonly __wbg_didresolutionrequestview_free: (a: number, b: number) => void;
    readonly __wbg_didresolutionresultview_free: (a: number, b: number) => void;
    readonly diddereferencingrequestview_did: (a: number) => [number, number];
    readonly diddereferencingrequestview_didUrl: (a: number) => [number, number];
    readonly diddereferencingrequestview_fragment: (a: number) => [number, number];
    readonly diddereferencingrequestview_method: (a: number) => [number, number];
    readonly diddereferencingrequestview_methodSpecificId: (a: number) => [number, number];
    readonly diddereferencingrequestview_options: (a: number) => [number, number];
    readonly diddereferencingrequestview_path: (a: number) => [number, number];
    readonly diddereferencingrequestview_query: (a: number) => [number, number];
    readonly diddereferencingresultview_kind: (a: number) => [number, number];
    readonly diddereferencingresultview_serialized: (a: number) => [number, number];
    readonly didresolutionrequestview_did: (a: number) => [number, number];
    readonly didresolutionrequestview_method: (a: number) => [number, number];
    readonly didresolutionrequestview_methodSpecificId: (a: number) => [number, number];
    readonly didresolutionrequestview_options: (a: number) => [number, number];
    readonly didresolutionresultview_kind: (a: number) => [number, number];
    readonly didresolutionresultview_serialized: (a: number) => [number, number];
    readonly parseDidDereferencingResult: (a: number, b: number) => [number, number, number];
    readonly parseDidResolutionResult: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly prepareDidDereferencing: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly prepareDidResolution: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly __wbg_diddocumentview_free: (a: number, b: number) => void;
    readonly __wbg_didparseerror_free: (a: number, b: number) => void;
    readonly __wbg_didurlview_free: (a: number, b: number) => void;
    readonly __wbg_didview_free: (a: number, b: number) => void;
    readonly bindingApiVersion: () => number;
    readonly diddocumentview_id: (a: number) => [number, number];
    readonly diddocumentview_serialized: (a: number) => [number, number];
    readonly didparseerror_code: (a: number) => [number, number];
    readonly didurlview_did: (a: number) => [number, number];
    readonly didurlview_fragment: (a: number) => [number, number];
    readonly didurlview_method: (a: number) => [number, number];
    readonly didurlview_methodSpecificId: (a: number) => [number, number];
    readonly didurlview_path: (a: number) => [number, number];
    readonly didurlview_query: (a: number) => [number, number];
    readonly didurlview_value: (a: number) => [number, number];
    readonly didview_method: (a: number) => [number, number];
    readonly didview_methodSpecificId: (a: number) => [number, number];
    readonly didview_value: (a: number) => [number, number];
    readonly parseDid: (a: number, b: number) => [number, number, number];
    readonly parseDidDocument: (a: number, b: number) => [number, number, number];
    readonly parseDidUrl: (a: number, b: number) => [number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
