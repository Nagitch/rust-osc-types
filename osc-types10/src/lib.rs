#![forbid(unsafe_code)]
#![deny(missing_docs, unreachable_pub, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = r#"
# osc-types10
**⚠ Experimental / Not for production use**

Defines message and bundle types for Open Sound Control 1.0.
- Pre-release (`0.1.0-alpha`)
- Breaking changes may occur frequently
- `no_std` compatible (optional)

## Example
```rust
use osc_types10::Message;
let msg = Message::new("/example", vec!["abc".into()]);
println!("{msg:?}");
```
"#]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Example placeholder type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<'a> {
    /// OSC address
    pub address: &'a str,
    /// Arguments
    pub args: Vec<&'a str>,
}

impl<'a> Message<'a> {
    /// Create a new OSC message
    pub fn new(address: &'a str, args: Vec<&'a str>) -> Self {
        Self { address, args }
    }
}

/// Example placeholder type for OSC bundles
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundle<'a> {
    /// OSC time tag
    pub timetag: u64,
    /// Messages contained in the bundle
    pub messages: Vec<Message<'a>>,
}

impl<'a> Bundle<'a> {
    /// Create a new OSC bundle
    pub fn new(timetag: u64, messages: Vec<Message<'a>>) -> Self {
        Self { timetag, messages }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bundle, Message};

    #[test]
    fn message_new_sets_address_and_args() {
        let msg = Message::new("/test", vec!["one", "two"]);

        assert_eq!(msg.address, "/test");
        assert_eq!(msg.args, vec!["one", "two"]);
    }

    #[test]
    fn message_equality_compares_contents() {
        let lhs = Message::new("/foo", vec!["a", "b"]);
        let rhs = Message::new("/foo", vec!["a", "b"]);
        let different_address = Message::new("/bar", vec!["a", "b"]);
        let different_args = Message::new("/foo", vec!["a"]);

        assert_eq!(lhs, rhs);
        assert_ne!(lhs, different_address);
        assert_ne!(lhs, different_args);
    }

    #[test]
    fn bundle_new_sets_timetag_and_messages() {
        let messages = vec![
            Message::new("/bundle/one", vec!["1"]),
            Message::new("/bundle/two", vec!["2"]),
        ];
        let bundle = Bundle::new(42, messages.clone());

        assert_eq!(bundle.timetag, 42);
        assert_eq!(bundle.messages, messages);
    }

    #[test]
    fn bundle_equality_compares_contents() {
        let messages = vec![
            Message::new("/bundle", vec!["a"]),
            Message::new("/bundle", vec!["b"]),
        ];
        let lhs = Bundle::new(1, messages.clone());
        let rhs = Bundle::new(1, messages);
        let different_timetag = Bundle::new(2, vec![Message::new("/bundle", vec!["a"])]);
        let different_messages = Bundle::new(1, vec![Message::new("/bundle", vec!["c"])]);

        assert_eq!(lhs, rhs);
        assert_ne!(lhs, different_timetag);
        assert_ne!(lhs, different_messages);
    }
}
