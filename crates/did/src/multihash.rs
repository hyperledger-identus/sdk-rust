//! `Multihash` — a method-agnostic multihash bytes domain primitive.
//!
//! Dogfoods the bytes category of `#[derive(Newtype)]`: `Multihash(Vec<u8>)`
//! gets `as_bytes`, `into_bytes`, `AsRef<[u8]>`, `From<Vec<u8>>`/`From<&[u8]>`,
//! a hex `Display`, and transparent serde (emitting/accepting a hex JSON
//! string, matching `Display`). This is the canonical method-agnostic bytes
//! identifier underlying `did:key` (and the wider multihash/multibase
//! ecosystem); it is not PRISM-specific. No `parse`/`FromStr` is generated in
//! this adoption (a validated multihash code/length/digest structure check is a
//! follow-on, not a v1 dogfood requirement).

use identus_derive::Newtype;

/// A multihash as raw bytes.
///
/// Construct infallibly with [`Multihash::new`] or [`Multihash::from`]; access
/// the bytes with [`Multihash::as_bytes`] / [`AsRef<[u8]>`] / take them with
/// [`Multihash::into_bytes`]. `Display` and serde render the bytes as
/// lowercase hex.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display = "hex", serde)]
pub struct Multihash(Vec<u8>);

#[cfg(test)]
mod tests {
    use super::*;

    // A did:key-style multihash for the identity Ed25519 public key
    // `did:key:z6Mk...`: the multihash here is the raw bytes form (varint code
    // + varint length + digest), exercised method-agnostically.
    const SAMPLE: &[u8] = &[0x00, 0x20];

    #[test]
    fn multihash_accessors_and_conversions() {
        let mh = Multihash::from(SAMPLE);
        assert_eq!(mh.as_bytes(), SAMPLE);
        assert_eq!(<Multihash as AsRef<[u8]>>::as_ref(&mh), SAMPLE);
        assert_eq!(Multihash::new(SAMPLE.to_vec()).as_bytes(), SAMPLE);
        assert_eq!(mh.into_bytes(), SAMPLE.to_vec());
    }

    #[test]
    fn multihash_display_renders_hex() {
        let mh = Multihash::from(SAMPLE);
        assert_eq!(mh.to_string(), "0020");
        let mh2 = Multihash::new(vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(mh2.to_string(), "deadbeef");
    }

    #[test]
    fn multihash_serde_roundtrips_as_hex_string() {
        let mh = Multihash::new(vec![0x01, 0x02, 0x03]);
        let json = serde_json::to_string(&mh).unwrap();
        assert_eq!(json, "\"010203\"");
        let back: Multihash = serde_json::from_str("\"010203\"").unwrap();
        assert_eq!(back, mh);
    }
}
