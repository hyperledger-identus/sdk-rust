import {
  createContext,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";

import type { DidProvider, DidProviderFactory } from "./public.js";

export type DidProviderState =
  | Readonly<{ status: "loading" }>
  | Readonly<{ status: "ready"; provider: DidProvider }>
  | Readonly<{ status: "failed"; error: Error }>;

export interface DidProviderBoundaryProps {
  readonly factory: DidProviderFactory;
  readonly children: ReactNode;
  readonly initialProvider?: DidProvider;
}

const loadingState: DidProviderState = Object.freeze({ status: "loading" });
const DidContext = createContext<DidProviderState | undefined>(undefined);

/** Own provider initialization and React lifecycle without parsing semantics. */
export function DidProviderBoundary({
  factory,
  children,
  initialProvider,
}: DidProviderBoundaryProps): ReactNode {
  const initialState = useMemo<DidProviderState>(
    () => initialProvider === undefined
      ? loadingState
      : Object.freeze({ status: "ready", provider: initialProvider }),
    [initialProvider],
  );
  const [state, setState] = useState<DidProviderState>(initialState);

  useEffect(() => {
    if (initialProvider !== undefined) {
      setState(Object.freeze({ status: "ready", provider: initialProvider }));
      return undefined;
    }
    let active = true;
    setState(loadingState);
    void factory().then(
      (provider) => {
        if (active) {
          setState(Object.freeze({ status: "ready", provider }));
        }
      },
      () => {
        if (active) {
          setState(Object.freeze({
            status: "failed",
            error: Object.freeze(new Error("DID provider initialization failed")),
          }));
        }
      },
    );
    return () => {
      active = false;
    };
  }, [factory, initialProvider]);

  return <DidContext.Provider value={state}>{children}</DidContext.Provider>;
}

export function useDidProvider(): DidProviderState {
  const state = useContext(DidContext);
  if (state === undefined) {
    throw new Error("useDidProvider requires DidProviderBoundary");
  }
  return state;
}
