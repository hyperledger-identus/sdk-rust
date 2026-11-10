import type {
  DidDocumentContextEntryValue,
  DidDocumentContextValue,
  DidDocumentControllerValue,
  DidDocumentExtensionMapValue,
  DidDocumentExtensionValue,
  DidDocumentValue,
  DidProvider,
  DidProviderFactory,
  DidServiceEndpointEntryValue,
  DidServiceEndpointValue,
  DidServiceValue,
  DidUrlValue,
  DidValue,
  DidVerificationMethodValue,
  DidVerificationRelationshipValue,
} from "../src/index.js";
import { createDidProvider as createBrowserDidProvider } from "../src/browser.js";
import { createDidProvider as createNodeDidProvider } from "../src/node.js";
import type { DidProviderState } from "../src/react.js";

type IsAny<T> = 0 extends (1 & T) ? true : false;
type AssertFalse<T extends false> = T;

type DidIsNotAny = AssertFalse<IsAny<DidValue>>;
type DidUrlIsNotAny = AssertFalse<IsAny<DidUrlValue>>;
type ProviderIsNotAny = AssertFalse<IsAny<DidProvider>>;
type DocumentIsNotAny = AssertFalse<IsAny<DidDocumentValue>>;
type DocumentContextIsNotAny = AssertFalse<IsAny<DidDocumentContextValue>>;
type DocumentContextEntryIsNotAny = AssertFalse<IsAny<DidDocumentContextEntryValue>>;
type DocumentControllerIsNotAny = AssertFalse<IsAny<DidDocumentControllerValue>>;
type DocumentExtensionIsNotAny = AssertFalse<IsAny<DidDocumentExtensionValue>>;
type DocumentExtensionMapIsNotAny = AssertFalse<IsAny<DidDocumentExtensionMapValue>>;
type VerificationMethodIsNotAny = AssertFalse<IsAny<DidVerificationMethodValue>>;
type VerificationRelationshipIsNotAny = AssertFalse<IsAny<DidVerificationRelationshipValue>>;
type ServiceIsNotAny = AssertFalse<IsAny<DidServiceValue>>;
type ServiceEndpointIsNotAny = AssertFalse<IsAny<DidServiceEndpointValue>>;
type ServiceEndpointEntryIsNotAny = AssertFalse<IsAny<DidServiceEndpointEntryValue>>;
type FactoryIsNotAny = AssertFalse<IsAny<DidProviderFactory>>;
type BrowserFactoryIsNotAny = AssertFalse<IsAny<typeof createBrowserDidProvider>>;
type NodeFactoryIsNotAny = AssertFalse<IsAny<typeof createNodeDidProvider>>;
type ReactStateIsNotAny = AssertFalse<IsAny<DidProviderState>>;

const compileAssertions: readonly [
  DidIsNotAny,
  DidUrlIsNotAny,
  ProviderIsNotAny,
  DocumentIsNotAny,
  DocumentContextIsNotAny,
  DocumentContextEntryIsNotAny,
  DocumentControllerIsNotAny,
  DocumentExtensionIsNotAny,
  DocumentExtensionMapIsNotAny,
  VerificationMethodIsNotAny,
  VerificationRelationshipIsNotAny,
  ServiceIsNotAny,
  ServiceEndpointIsNotAny,
  ServiceEndpointEntryIsNotAny,
  FactoryIsNotAny,
  BrowserFactoryIsNotAny,
  NodeFactoryIsNotAny,
  ReactStateIsNotAny,
] = [
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
];

declare const document: DidDocumentValue;
const documentId: string = document.id.serialized;
const documentJson: string = document.serialized;
const extension: DidDocumentExtensionValue | undefined = document.properties.example;
const relationship: DidVerificationRelationshipValue | undefined = document.authentication?.[0];
const relationshipId: string | undefined = relationship?.kind === "reference"
  ? relationship.reference
  : relationship?.method.id;

void documentId;
void documentJson;
void extension;
void relationshipId;

void compileAssertions;
