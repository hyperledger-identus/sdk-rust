/* tslint:disable */
/* eslint-disable */

/**
 * Closed browser failure carrying only a stable SDK-owned code.
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
 * Return the browser binding contract version.
 */
export function bindingApiVersion(): number;

/**
 * Parse a bounded DID into an owned JavaScript view.
 */
export function parseDid(value: string): DidView;

/**
 * Parse a bounded DID URL into an owned JavaScript view.
 */
export function parseDidUrl(value: string): DidUrlView;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_didparseerror_free: (a: number, b: number) => void;
    readonly __wbg_didurlview_free: (a: number, b: number) => void;
    readonly __wbg_didview_free: (a: number, b: number) => void;
    readonly bindingApiVersion: () => number;
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
    readonly parseDidUrl: (a: number, b: number) => [number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
