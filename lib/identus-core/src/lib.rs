//! Identus Core — foundational types and utilities for the Identus Rust SDK.
//!
//! This crate provides the core building blocks for the SDK ecosystem.
#![doc(html_root_url = "https://docs.rs/identus-core")]

pub fn hello() -> &'static str {
    "Hello, Identus!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, Identus!");
    }
}
