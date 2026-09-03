//! Deterministic immutable composition and dispatch for DID method adapters.
//!
//! The registry owns no method, VDR, transport, cache or wallet policy. It
//! freezes validated method bindings at setup and implements the existing W3C
//! query ports through exact borrowed method-name lookup.

use std::{collections::BTreeMap, fmt, sync::Arc};

use crate::{
    DereferencingOptions, Did, DidMethod, DidRegistrar, DidRegistrationErrorKind,
    DidRegistrationFuture, DidRegistrationResult, DidResolutionError, DidResolutionErrorKind,
    DidResolutionFuture, DidResolutionMetadata, DidResolutionResult, DidResolver, DidUrl,
    DidUrlDereferencer, DidUrlDereferencingFuture, DidUrlDereferencingMetadata,
    DidUrlDereferencingResult, Error, RegistrationRequest, error::RegistryError,
};

/// Maximum number of method bindings in one immutable registry.
pub const MAX_DID_METHOD_REGISTRY_ENTRIES: usize = 64;

/// One validated DID method's query capabilities.
#[derive(Clone)]
pub struct DidMethodBinding {
    method: DidMethod,
    resolver: Arc<dyn DidResolver>,
    dereferencer: Option<Arc<dyn DidUrlDereferencer>>,
    registrar: Option<Arc<dyn DidRegistrar>>,
}

impl DidMethodBinding {
    /// Bind a validated method name to its required resolver.
    #[must_use]
    pub fn new(method: DidMethod, resolver: Arc<dyn DidResolver>) -> Self {
        Self {
            method,
            resolver,
            dereferencer: None,
            registrar: None,
        }
    }

    /// Add this method's independent DID URL dereferencing capability.
    #[must_use]
    pub fn with_dereferencer(mut self, dereferencer: Arc<dyn DidUrlDereferencer>) -> Self {
        self.dereferencer = Some(dereferencer);
        self
    }

    /// Add this method's independent DID Registration capability.
    #[must_use]
    pub fn with_registrar(mut self, registrar: Arc<dyn DidRegistrar>) -> Self {
        self.registrar = Some(registrar);
        self
    }

    /// Borrow the exact validated method name.
    #[must_use]
    pub const fn method(&self) -> &DidMethod {
        &self.method
    }

    /// Whether this binding independently supports DID URL dereferencing.
    #[must_use]
    pub const fn supports_dereferencing(&self) -> bool {
        self.dereferencer.is_some()
    }

    /// Whether this binding independently supports DID Registration.
    #[must_use]
    pub const fn supports_registration(&self) -> bool {
        self.registrar.is_some()
    }
}

impl fmt::Debug for DidMethodBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DidMethodBinding")
            .field("method", &self.method)
            .field("supports_dereferencing", &self.supports_dereferencing())
            .field("supports_registration", &self.supports_registration())
            .finish_non_exhaustive()
    }
}

/// Mutable setup surface for an immutable [`DidMethodRegistry`].
#[derive(Clone, Default)]
pub struct DidMethodRegistryBuilder {
    bindings: BTreeMap<String, DidMethodBinding>,
}

impl DidMethodRegistryBuilder {
    /// Register exactly one owner for a DID method.
    pub fn register(mut self, binding: DidMethodBinding) -> Result<Self, Error> {
        let key = binding.method().as_str();
        if self.bindings.contains_key(key) {
            return Err(invalid(RegistryError::DuplicateMethod));
        }
        if self.bindings.len() >= MAX_DID_METHOD_REGISTRY_ENTRIES {
            return Err(invalid(RegistryError::TooManyMethods));
        }
        self.bindings.insert(key.to_owned(), binding);
        Ok(self)
    }

    /// Freeze all registered bindings into a shared immutable lookup table.
    #[must_use]
    pub fn build(self) -> DidMethodRegistry {
        DidMethodRegistry {
            bindings: Arc::new(self.bindings),
        }
    }
}

impl fmt::Debug for DidMethodRegistryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DidMethodRegistryBuilder")
            .field("methods", &self.bindings.keys().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

/// Immutable deterministic dispatcher for registered DID methods.
#[derive(Clone, Default)]
pub struct DidMethodRegistry {
    bindings: Arc<BTreeMap<String, DidMethodBinding>>,
}

impl DidMethodRegistry {
    /// Begin bounded registry construction.
    #[must_use]
    pub fn builder() -> DidMethodRegistryBuilder {
        DidMethodRegistryBuilder::default()
    }

    /// Return a valid registry with no method bindings.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Return the number of registered methods.
    #[must_use]
    pub fn method_count(&self) -> usize {
        self.bindings.len()
    }

    /// Whether no methods are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Whether the exact validated method has a binding.
    #[must_use]
    pub fn contains_method(&self, method: &DidMethod) -> bool {
        self.bindings.contains_key(method.as_str())
    }

    /// Whether the exact validated method has a dereferencing capability.
    #[must_use]
    pub fn supports_dereferencing(&self, method: &DidMethod) -> bool {
        self.bindings
            .get(method.as_str())
            .is_some_and(DidMethodBinding::supports_dereferencing)
    }

    /// Whether the exact validated method has a registration capability.
    #[must_use]
    pub fn supports_registration(&self, method: &DidMethod) -> bool {
        self.bindings
            .get(method.as_str())
            .is_some_and(DidMethodBinding::supports_registration)
    }

    /// Iterate registered method names in deterministic lexical order.
    pub fn methods(&self) -> impl ExactSizeIterator<Item = &str> {
        self.bindings.keys().map(String::as_str)
    }
}

impl fmt::Debug for DidMethodRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DidMethodRegistry")
            .field("methods", &self.bindings.keys().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

impl DidResolver for DidMethodRegistry {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a crate::ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        self.bindings.get(did.method()).map_or_else(
            || {
                Box::pin(async { resolution_failure(DidResolutionErrorKind::MethodNotSupported) })
                    as DidResolutionFuture<'a>
            },
            |binding| binding.resolver.resolve(did, options),
        )
    }
}

impl DidUrlDereferencer for DidMethodRegistry {
    fn dereference<'a>(
        &'a self,
        did_url: &'a DidUrl,
        options: &'a DereferencingOptions,
    ) -> DidUrlDereferencingFuture<'a> {
        match self.bindings.get(did_url.method()) {
            Some(binding) => binding.dereferencer.as_ref().map_or_else(
                || {
                    Box::pin(async {
                        dereferencing_failure(DidResolutionErrorKind::FeatureNotSupported)
                    }) as DidUrlDereferencingFuture<'a>
                },
                |dereferencer| dereferencer.dereference(did_url, options),
            ),
            None => Box::pin(async {
                dereferencing_failure(DidResolutionErrorKind::MethodNotSupported)
            }),
        }
    }
}

impl DidRegistrar for DidMethodRegistry {
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a> {
        match self.bindings.get(request.method().as_str()) {
            Some(binding) => binding.registrar.as_ref().map_or_else(
                || {
                    let method = request.method().clone();
                    Box::pin(async move {
                        DidRegistrationResult::standard_failure(
                            method,
                            DidRegistrationErrorKind::FeatureNotSupported,
                        )
                    }) as DidRegistrationFuture<'a>
                },
                |registrar| registrar.execute(request),
            ),
            None => {
                let method = request.method().clone();
                Box::pin(async move {
                    DidRegistrationResult::standard_failure(
                        method,
                        DidRegistrationErrorKind::MethodNotSupported,
                    )
                })
            }
        }
    }
}

fn resolution_failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard error metadata is valid");
    DidResolutionResult::failure(metadata).expect("standard resolution failure is valid")
}

fn dereferencing_failure(kind: DidResolutionErrorKind) -> DidUrlDereferencingResult {
    let metadata = DidUrlDereferencingMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard error metadata is valid");
    DidUrlDereferencingResult::failure(metadata).expect("standard dereferencing failure is valid")
}

const fn invalid(reason: RegistryError) -> Error {
    Error::InvalidRegistry(reason)
}
