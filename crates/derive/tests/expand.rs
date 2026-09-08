//! Per-category runtime expansion checks for `#[derive(Newtype)]`.
//!
//! Exercises the generated constructors, accessors, conversions, `Display`,
//! serde, and fallible `parse` for the string category, plus the accessors,
//! conversions, `Display`, and serde for the bytes and numeric categories.
//! `parse`/`FromStr` is string-only; numeric and bytes drop the string-shaped
//! entry under `validate_fn`.

use identus_derive::Newtype;
use std::{cell::Cell, str::FromStr};

// --- string category (no validate_fn: infallible construction) -----------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display, serde)]
struct Plain(String);

#[test]
fn str_newtype_infallible_when_unvalidated() {
    let t = Plain::new("hello".to_owned());
    assert_eq!(t.as_str(), "hello");
    assert_eq!(<Plain as AsRef<str>>::as_ref(&t), "hello");
    assert_eq!(Plain::from("world".to_owned()).as_str(), "world");
    assert_eq!(Plain::from("bye").as_str(), "bye");
    assert_eq!(t.to_string(), "hello");
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json, "\"hello\"");
    let back: Plain = serde_json::from_str("\"hello\"").unwrap();
    assert_eq!(back, t);
}

// --- string category (validate_fn: validating construction) --------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display, serde, validate_fn = validate_nonempty, validate_err = EmptyError)]
struct Tag(String);

#[derive(Debug)]
struct EmptyError;

impl std::fmt::Display for EmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("empty string")
    }
}

fn validate_nonempty(s: &str) -> Result<(), EmptyError> {
    if s.is_empty() {
        Err(EmptyError)
    } else {
        Ok(())
    }
}

#[test]
fn str_newtype_accessors_and_conversions() {
    let t = Tag::try_new("hello".to_owned()).unwrap();
    assert_eq!(t.as_str(), "hello");
    assert_eq!(<Tag as AsRef<str>>::as_ref(&t), "hello");
    assert_eq!(Tag::try_from("world".to_owned()).unwrap().as_str(), "world");
    assert_eq!(Tag::parse("bye").unwrap().as_str(), "bye");
    assert_eq!(t.to_string(), "hello");
}

#[test]
fn str_newtype_parse_and_fromstr() {
    let parsed: Tag = "hi".parse().unwrap();
    assert_eq!(parsed.as_str(), "hi");
    assert_eq!(Tag::parse("ok").unwrap().as_str(), "ok");
    assert!(Tag::from_str("").is_err());
    assert!(Tag::parse("").is_err());
}

thread_local! {
    static EXPECTED_BORROWED_PTR: Cell<usize> = const { Cell::new(0) };
}

#[derive(Clone, Debug, PartialEq, Eq, Newtype)]
#[newtype(validate_fn = validate_borrowed, validate_err = BorrowedError)]
struct BorrowedTag(String);

#[derive(Debug)]
struct BorrowedError;

fn validate_borrowed(value: &str) -> Result<(), BorrowedError> {
    EXPECTED_BORROWED_PTR.with(|expected| {
        (expected.get() == value.as_ptr() as usize)
            .then_some(())
            .ok_or(BorrowedError)
    })
}

#[test]
fn str_newtype_borrowed_paths_validate_before_allocation() {
    let input = "borrowed-input".to_owned();
    EXPECTED_BORROWED_PTR.with(|expected| expected.set(input.as_ptr() as usize));

    let parsed = BorrowedTag::parse(input.as_str()).unwrap();
    assert_eq!(parsed.as_str(), input);

    let from_str = BorrowedTag::from_str(input.as_str()).unwrap();
    assert_eq!(from_str.as_str(), input);

    EXPECTED_BORROWED_PTR.with(|expected| expected.set(0));
}

#[test]
fn str_newtype_serde_roundtrip() {
    let t = Tag::try_new("hello".to_owned()).unwrap();
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json, "\"hello\"");
    let back: Tag = serde_json::from_str("\"hello\"").unwrap();
    assert_eq!(back, t);
    // Empty string fails `validate_nonempty`; validating `Deserialize` rejects.
    assert!(serde_json::from_str::<Tag>("\"\"").is_err());
}

// --- bytes category (hex) ---------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Newtype)]
#[newtype(display, serde)]
struct Hash(Vec<u8>);

#[test]
fn bytes_hex_accessors_and_conversions() {
    let h = Hash::new(vec![0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(h.as_bytes(), &[0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(<Hash as AsRef<[u8]>>::as_ref(&h), &[0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(Hash::from(&[0u8, 1][..]).as_bytes(), &[0, 1]);
    assert_eq!(Hash::from(vec![2, 3]).into_bytes(), vec![2, 3]);
}

#[test]
fn bytes_hex_display() {
    assert_eq!(
        Hash::new(vec![0xde, 0xad, 0xbe, 0xef]).to_string(),
        "deadbeef"
    );
    assert_eq!(Hash::new(vec![]).to_string(), "");
}

#[test]
fn bytes_hex_serde_roundtrip() {
    let h = Hash::new(vec![1, 2, 3]);
    let json = serde_json::to_string(&h).unwrap();
    assert_eq!(json, "\"010203\"");
    let back: Hash = serde_json::from_str("\"010203\"").unwrap();
    assert_eq!(back, h);
    assert!(serde_json::from_str::<Hash>("\"zz\"").is_err());
    assert!(serde_json::from_str::<Hash>("\"abc\"").is_err()); // odd length
}

// --- bytes category (base64url) --------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Newtype)]
#[newtype(display = "base64url", serde)]
struct B64(Vec<u8>);

#[test]
fn bytes_base64url_display_and_serde() {
    // [b'f', b'o'] -> base64url (no padding) "Zm8"
    let b = B64::new(vec![b'f', b'o']);
    assert_eq!(b.to_string(), "Zm8");
    let json = serde_json::to_string(&b).unwrap();
    assert_eq!(json, "\"Zm8\"");
    let back: B64 = serde_json::from_str("\"Zm8\"").unwrap();
    assert_eq!(back, b);
    let empty = B64::new(vec![]);
    assert_eq!(empty.to_string(), "");
    assert_eq!(serde_json::to_string(&empty).unwrap(), "\"\"");
    // three bytes -> four chars, no padding
    let three = B64::new(vec![0, 0, 0]);
    assert_eq!(three.to_string(), "AAAA");
}

// --- bytes category (validate_fn: validating construction) ---------------

#[derive(Clone, Debug, PartialEq, Eq, Newtype)]
#[newtype(display, serde, validate_fn = validate_nonempty_bytes, validate_err = EmptyBytesError)]
struct NonEmpty(Vec<u8>);

#[derive(Debug)]
struct EmptyBytesError;

impl std::fmt::Display for EmptyBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("empty bytes")
    }
}

fn validate_nonempty_bytes(v: &[u8]) -> Result<(), EmptyBytesError> {
    if v.is_empty() {
        Err(EmptyBytesError)
    } else {
        Ok(())
    }
}

#[test]
fn bytes_newtype_try_new_validates() {
    assert_eq!(
        NonEmpty::try_new(vec![0xde, 0xad]).unwrap().as_bytes(),
        &[0xde, 0xad]
    );
    assert_eq!(NonEmpty::try_from(vec![0x01]).unwrap().as_bytes(), &[0x01]);
    // Validation failure does not construct the value.
    assert!(NonEmpty::try_new(vec![]).is_err());
    assert!(NonEmpty::try_from(vec![]).is_err());
    // `pub(crate)` hatch: callable from the defining crate, skips validation.
    assert_eq!(NonEmpty::new_unchecked(vec![]).as_bytes(), &[] as &[u8]);
}

#[test]
fn bytes_newtype_serde_decodes_then_validates() {
    // Valid hex decoding to non-empty bytes: round-trips.
    let n = NonEmpty::try_new(vec![0x01, 0x02]).unwrap();
    let json = serde_json::to_string(&n).unwrap();
    assert_eq!(json, "\"0102\"");
    let back: NonEmpty = serde_json::from_str("\"0102\"").unwrap();
    assert_eq!(back, n);

    // Malformed hex: decode failure is a first-class `Err`, no panic, and the
    // validator is never reached.
    assert!(serde_json::from_str::<NonEmpty>("\"zz\"").is_err());
    assert!(serde_json::from_str::<NonEmpty>("\"abc\"").is_err()); // odd length

    // Valid hex decoding to empty bytes: decode succeeds but validation fails.
    assert!(serde_json::from_str::<NonEmpty>("\"\"").is_err());
}

// --- numeric category (no validate_fn: infallible construction) -----------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Newtype)]
#[newtype(display, serde)]
struct Port(u16);

#[test]
fn num_newtype_accessors_and_conversions() {
    let p = Port::new(8080);
    assert_eq!(p.get(), 8080);
    assert_eq!(Port::from(443).get(), 443);
    assert_eq!(p.to_string(), "8080");
}

#[test]
fn num_newtype_serde_roundtrip() {
    let p = Port::new(8080);
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, "8080");
    let back: Port = serde_json::from_str("8080").unwrap();
    assert_eq!(back, p);
}

// --- numeric category (validate_fn: validating construction) --------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Newtype)]
#[newtype(display, serde, validate_fn = validate_bounded_port, validate_err = PortError)]
struct BoundedPort(u16);

#[derive(Debug)]
struct PortError;

impl std::fmt::Display for PortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid port")
    }
}

fn validate_bounded_port(n: &u16) -> Result<(), PortError> {
    if *n > 0 { Ok(()) } else { Err(PortError) }
}

#[test]
fn num_newtype_try_new_validates() {
    assert_eq!(BoundedPort::try_new(8080).unwrap().get(), 8080);
    assert_eq!(BoundedPort::try_from(443).unwrap().get(), 443);
    assert!(BoundedPort::try_new(0).is_err());
    assert!(BoundedPort::try_from(0).is_err());
    // `pub(crate)` hatch: callable from the defining crate, skips validation.
    assert_eq!(BoundedPort::new_unchecked(0).get(), 0);
}

#[test]
fn num_newtype_serde_validates() {
    assert_eq!(
        serde_json::from_str::<BoundedPort>("8080").unwrap().get(),
        8080
    );
    // numeric `Deserialize` now validates; zero fails the validator.
    assert!(serde_json::from_str::<BoundedPort>("0").is_err());
}
