//! BIP-32 derivation-path helpers: [`DerivationAxis`] and [`DerivationPath`].
//!
//! Ported from the KMP `derivation.DerivationAxis` / `derivation.DerivationPath`.
//! Indices between 0 and 2^31-1 are normal; the hardened flag sets bit 31.

use crate::derivation::{MAX_DERIVATION_PATH_AXES, MAX_DERIVATION_PATH_BYTES};
use crate::error::Error;

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

    /// The number of child axes in this path.
    pub fn len(&self) -> usize {
        self.axes.len()
    }

    /// Whether this path represents the root without child axes.
    pub fn is_empty(&self) -> bool {
        self.axes.is_empty()
    }

    /// Append `axis`, returning a new child path.
    ///
    /// This source-compatible builder is caller-budgeted and can construct a
    /// path above [`MAX_DERIVATION_PATH_AXES`]. Cryptographic path consumers
    /// reject such values before performing child-derivation work.
    pub fn derive(&self, axis: DerivationAxis) -> Self {
        let mut axes = self.axes.clone();
        axes.push(axis);
        Self { axes }
    }

    /// Parse a path string of the form `m/axis1/.../axisn` (each axis is a
    /// number optionally followed by `'` for hardened).
    ///
    /// Input is limited to [`MAX_DERIVATION_PATH_BYTES`] UTF-8 bytes and
    /// [`MAX_DERIVATION_PATH_AXES`] axes. Both excess cases fail before work
    /// on the rejected portion.
    pub fn from_path(path: &str) -> Result<Self, Error> {
        if path.len() > MAX_DERIVATION_PATH_BYTES {
            return Err(Error::DerivationFailed);
        }

        let mut parts = path.split('/');
        if parts
            .next()
            .map(str::trim)
            .map(|root| root.eq_ignore_ascii_case("m"))
            != Some(true)
        {
            return Err(Error::DerivationFailed);
        }

        let mut axes = Vec::with_capacity(parts.size_hint().0.min(MAX_DERIVATION_PATH_AXES));
        for part in parts {
            if axes.len() == MAX_DERIVATION_PATH_AXES {
                return Err(Error::DerivationFailed);
            }
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

    pub(crate) fn ensure_work_bound(&self) -> Result<(), Error> {
        if self.axes.len() > MAX_DERIVATION_PATH_AXES {
            return Err(Error::DerivationFailed);
        }
        Ok(())
    }

    pub(crate) fn ensure_depth_capacity(&self, current_depth: u32) -> Result<(), Error> {
        let remaining_depth = u32::try_from(MAX_DERIVATION_PATH_AXES)
            .expect("the derivation-axis maximum fits in u32")
            .checked_sub(current_depth)
            .ok_or(Error::DerivationFailed)?;
        let additional_depth =
            u32::try_from(self.axes.len()).map_err(|_| Error::DerivationFailed)?;
        if additional_depth > remaining_depth {
            return Err(Error::DerivationFailed);
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn repeated_path(axis: &str, count: usize) -> String {
        let mut path = String::from("m");
        for _ in 0..count {
            path.push('/');
            path.push_str(axis);
        }
        path
    }

    #[test]
    fn text_byte_limit_is_checked_before_path_syntax() {
        let exact = format!("{}m", " ".repeat(MAX_DERIVATION_PATH_BYTES - 1));
        assert!(DerivationPath::from_path(&exact).unwrap().is_empty());

        let oversized_valid = format!("{}m", " ".repeat(MAX_DERIVATION_PATH_BYTES));
        assert!(matches!(
            DerivationPath::from_path(&oversized_valid),
            Err(Error::DerivationFailed)
        ));

        let oversized_invalid = "x".repeat(MAX_DERIVATION_PATH_BYTES + 1);
        assert!(matches!(
            DerivationPath::from_path(&oversized_invalid),
            Err(Error::DerivationFailed)
        ));
    }

    #[test]
    fn parser_accepts_255_axes_and_rejects_the_256th() {
        let maximum =
            DerivationPath::from_path(&repeated_path("0", MAX_DERIVATION_PATH_AXES)).unwrap();
        assert_eq!(maximum.len(), MAX_DERIVATION_PATH_AXES);
        assert!(!maximum.is_empty());

        assert!(matches!(
            DerivationPath::from_path(&repeated_path("0", MAX_DERIVATION_PATH_AXES + 1)),
            Err(Error::DerivationFailed)
        ));

        let invalid_excess = format!(
            "{}/not-an-axis",
            repeated_path("0", MAX_DERIVATION_PATH_AXES)
        );
        assert!(matches!(
            DerivationPath::from_path(&invalid_excess),
            Err(Error::DerivationFailed)
        ));
    }

    #[test]
    fn programmatic_oversized_path_is_rejected_by_work_preflight() {
        let mut path = DerivationPath::empty();
        for _ in 0..=MAX_DERIVATION_PATH_AXES {
            path = path.derive(DerivationAxis::normal(0));
        }

        assert_eq!(path.len(), MAX_DERIVATION_PATH_AXES + 1);
        assert!(matches!(
            path.ensure_work_bound(),
            Err(Error::DerivationFailed)
        ));
    }

    #[test]
    fn depth_capacity_uses_the_same_interoperable_ceiling() {
        let one = DerivationPath::from_path("m/0").unwrap();
        assert!(one.ensure_depth_capacity(254).is_ok());
        assert!(matches!(
            one.ensure_depth_capacity(255),
            Err(Error::DerivationFailed)
        ));
    }
}
