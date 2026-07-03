//! BIP-32 derivation-path helpers: [`DerivationAxis`] and [`DerivationPath`].
//!
//! Ported from the KMP `derivation.DerivationAxis` / `derivation.DerivationPath`.
//! Indices between 0 and 2^31-1 are normal; the hardened flag sets bit 31.

/// The hardened-index offset (2^31), per BIP-32.
pub const HARDENED_OFFSET: u32 = 0x8000_0000;

/// One axis (component) of a BIP-32 derivation path.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivationAxis(u32);

impl DerivationAxis {
    /// A normal (non-hardened) axis for `number` (must be `< 2^31`).
    pub fn normal(number: u32) -> Self {
        assert!(number < HARDENED_OFFSET, "normal axis must be < 2^31");
        Self(number)
    }

    /// A hardened axis for `number` (must be `< 2^31`).
    pub fn hardened(number: u32) -> Self {
        assert!(
            number < HARDENED_OFFSET,
            "hardened axis number must be < 2^31"
        );
        Self(number | HARDENED_OFFSET)
    }

    /// Whether this axis is hardened.
    pub const fn is_hardened(&self) -> bool {
        (self.0 & HARDENED_OFFSET) != 0
    }

    /// The axis number, stripped of the hardened bit.
    pub const fn number(&self) -> u32 {
        self.0 & !HARDENED_OFFSET
    }

    /// The raw (hardened-flagged) index.
    pub const fn raw(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for DerivationAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_hardened() {
            write!(f, "{}'", self.number())
        } else {
            write!(f, "{}", self.0)
        }
    }
}

/// A BIP-32 derivation path (a sequence of [`DerivationAxis`]es).
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationPath {
    axes: Vec<DerivationAxis>,
}

impl DerivationPath {
    /// An empty derivation path (the root `m`).
    pub fn empty() -> Self {
        Self { axes: Vec::new() }
    }

    /// The axes of this path.
    pub fn axes(&self) -> &[DerivationAxis] {
        &self.axes
    }

    /// Append `axis`, returning a new child path.
    pub fn derive(&self, axis: DerivationAxis) -> Self {
        let mut axes = self.axes.clone();
        axes.push(axis);
        Self { axes }
    }

    /// Parse a path string of the form `m/axis1/.../axisn` (each axis is a
    /// number optionally followed by `'` for hardened).
    pub fn from_path(path: &str) -> Result<Self, crate::error::Error> {
        use crate::error::Error;
        let parts: Vec<&str> = path.split('/').collect();
        if parts.first().map(|s| s.trim().eq_ignore_ascii_case("m")) != Some(true) {
            return Err(Error::DerivationFailed);
        }
        let mut axes = Vec::new();
        for part in parts.into_iter().skip(1) {
            let (num_str, hardened) = if let Some(stripped) = part.strip_suffix('\'') {
                (stripped, true)
            } else {
                (part, false)
            };
            let number: u32 = num_str.parse().map_err(|_| Error::DerivationFailed)?;
            if number >= HARDENED_OFFSET {
                return Err(Error::DerivationFailed);
            }
            axes.push(if hardened {
                DerivationAxis::hardened(number)
            } else {
                DerivationAxis::normal(number)
            });
        }
        Ok(Self { axes })
    }
}

impl std::fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("m")?;
        for axis in &self.axes {
            write!(f, "/{axis}")?;
        }
        Ok(())
    }
}
