//! Per-category runtime expansion checks for `#[derive(Newtype)]`.
//!
//! Exercises the generated constructors, accessors, conversions, `Display`,
//! serde, and fallible `parse` for the string, bytes, and numeric categories.

use identus_derive::Newtype;
use std::str::FromStr;

// --- string category -------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display, serde, parse = validate_nonempty, err = EmptyError)]
struct Tag(String);

#[derive(Debug)]
struct EmptyError;

fn validate_nonempty(s: &str) -> Result<(), EmptyError> {
    if s.is_empty() {
        Err(EmptyError)
    } else {
        Ok(())
    }
}

#[test]
fn str_newtype_accessors_and_conversions() {
    let t = Tag::new("hello".to_owned());
    assert_eq!(t.as_str(), "hello");
    assert_eq!(<Tag as AsRef<str>>::as_ref(&t), "hello");
    assert_eq!(Tag::from("world".to_owned()).as_str(), "world");
    assert_eq!(Tag::from("bye").as_str(), "bye");
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

#[test]
fn str_newtype_serde_roundtrip() {
    let t = Tag::new("hello".to_owned());
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json, "\"hello\"");
    let back: Tag = serde_json::from_str("\"hello\"").unwrap();
    assert_eq!(back, t);
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

// --- numeric category ------------------------------------------------------

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Newtype)]
#[newtype(display, serde, parse = validate_bounded_port, err = PortError)]
struct BoundedPort(u16);

#[derive(Debug)]
struct PortError;

fn validate_bounded_port(s: &str) -> Result<(), PortError> {
    let n: u16 = s.parse().map_err(|_| PortError)?;
    if n > 0 { Ok(()) } else { Err(PortError) }
}

#[test]
fn num_newtype_parse_validates() {
    assert_eq!(BoundedPort::parse("8080").unwrap().get(), 8080);
    let from_str: Result<BoundedPort, _> = "443".parse();
    assert_eq!(from_str.unwrap().get(), 443);
    assert!(BoundedPort::from_str("0").is_err());
    assert!(BoundedPort::from_str("notanumber").is_err());
}
