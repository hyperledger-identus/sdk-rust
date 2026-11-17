import type {
  DidProvider,
  DidProviderFactory,
  DidUrlValue,
  DidValue,
} from "../src/index.js";
import { createDidProvider as createBrowserDidProvider } from "../src/browser.js";
import { createDidProvider as createNodeDidProvider } from "../src/node.js";
import type { DidProviderState } from "../src/react.js";

type IsAny<T> = 0 extends (1 & T) ? true : false;
type AssertFalse<T extends false> = T;

type DidIsNotAny = AssertFalse<IsAny<DidValue>>;
type DidUrlIsNotAny = AssertFalse<IsAny<DidUrlValue>>;
type ProviderIsNotAny = AssertFalse<IsAny<DidProvider>>;
type FactoryIsNotAny = AssertFalse<IsAny<DidProviderFactory>>;
type BrowserFactoryIsNotAny = AssertFalse<IsAny<typeof createBrowserDidProvider>>;
type NodeFactoryIsNotAny = AssertFalse<IsAny<typeof createNodeDidProvider>>;
type ReactStateIsNotAny = AssertFalse<IsAny<DidProviderState>>;

const compileAssertions: readonly [
  DidIsNotAny,
  DidUrlIsNotAny,
  ProviderIsNotAny,
  FactoryIsNotAny,
  BrowserFactoryIsNotAny,
  NodeFactoryIsNotAny,
  ReactStateIsNotAny,
] = [false, false, false, false, false, false, false];

void compileAssertions;
