import { type ReactNode } from "react";
import type { DidProvider, DidProviderFactory } from "./public.js";
export type DidProviderState = Readonly<{
    status: "loading";
}> | Readonly<{
    status: "ready";
    provider: DidProvider;
}> | Readonly<{
    status: "failed";
    error: Error;
}>;
export interface DidProviderBoundaryProps {
    readonly factory: DidProviderFactory;
    readonly children: ReactNode;
    readonly initialProvider?: DidProvider;
}
/** Own provider initialization and React lifecycle without parsing semantics. */
export declare function DidProviderBoundary({ factory, children, initialProvider, }: DidProviderBoundaryProps): ReactNode;
export declare function useDidProvider(): DidProviderState;
