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
use osc_types10::{Message, OscType};
let msg = Message::new("/example", vec![OscType::String("abc")]);
println!("{msg:?}");
```
"#]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::{string::String, string::ToString, vec::Vec};

/// OSC argument types as defined in OSC 1.0 specification
#[derive(Debug, Clone, PartialEq)]
pub enum OscType<'a> {
    /// 32-bit integer (i)
    Int(i32),
    /// 32-bit IEEE 754 float (f)
    Float(f32),
    /// Null-terminated string (s)
    String(&'a str),
    /// Binary blob (b)
    Blob(&'a [u8]),
}

/// OSC Message as defined in OSC 1.0 specification
#[derive(Debug, Clone, PartialEq)]
pub struct Message<'a> {
    /// OSC address pattern
    pub address: &'a str,
    /// Type tag string indicating the types of the arguments
    pub type_tag: String,
    /// Arguments of the message
    pub args: Vec<OscType<'a>>,
}

impl<'a> Message<'a> {
    /// Create a new OSC message
    pub fn new(address: &'a str, args: Vec<OscType<'a>>) -> Self {
        // Generate type tag string from arguments
        let mut type_tag = ",".to_string();
        for arg in &args {
            match arg {
                OscType::Int(_) => type_tag.push('i'),
                OscType::Float(_) => type_tag.push('f'),
                OscType::String(_) => type_tag.push('s'),
                OscType::Blob(_) => type_tag.push('b'),
            }
        }

        Self {
            address,
            type_tag,
            args,
        }
    }

    /// Create a new OSC message with string arguments (convenience method)
    pub fn with_strings(address: &'a str, string_args: Vec<&'a str>) -> Self {
        let args = string_args.into_iter().map(OscType::String).collect();
        Self::new(address, args)
    }
}

/// Example placeholder type for OSC bundles
#[derive(Debug, Clone, PartialEq)]
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
    use super::{Bundle, Message, OscType};

    #[cfg(not(feature = "std"))]
    use alloc::vec;

    #[test]
    fn message_new_sets_address_and_args() {
        let msg = Message::with_strings("/test", vec!["one", "two"]);

        assert_eq!(msg.address, "/test");
        assert_eq!(
            msg.args,
            vec![OscType::String("one"), OscType::String("two")]
        );
    }

    #[test]
    fn message_equality_compares_contents() {
        let lhs = Message::with_strings("/foo", vec!["a", "b"]);
        let rhs = Message::with_strings("/foo", vec!["a", "b"]);
        let different_address = Message::with_strings("/bar", vec!["a", "b"]);
        let different_args = Message::with_strings("/foo", vec!["a"]);

        assert_eq!(lhs, rhs);
        assert_ne!(lhs, different_address);
        assert_ne!(lhs, different_args);
    }

    #[test]
    fn message_supports_mixed_types() {
        let msg = Message::new(
            "/mixed",
            vec![
                OscType::Int(42),
                OscType::Float(3.14),
                OscType::String("hello"),
                OscType::Blob(&[0x01, 0x02, 0x03]),
            ],
        );

        assert_eq!(msg.address, "/mixed");
        assert_eq!(msg.type_tag, ",ifsb");
        assert_eq!(msg.args.len(), 4);
    }

    #[test]
    fn bundle_new_sets_timetag_and_messages() {
        let messages = vec![
            Message::with_strings("/bundle/one", vec!["1"]),
            Message::with_strings("/bundle/two", vec!["2"]),
        ];
        let bundle = Bundle::new(42, messages.clone());

        assert_eq!(bundle.timetag, 42);
        assert_eq!(bundle.messages, messages);
    }

    #[test]
    fn bundle_equality_compares_contents() {
        let messages = vec![
            Message::with_strings("/bundle", vec!["a"]),
            Message::with_strings("/bundle", vec!["b"]),
        ];
        let lhs = Bundle::new(1, messages.clone());
        let rhs = Bundle::new(1, messages);
        let different_timetag = Bundle::new(2, vec![Message::with_strings("/bundle", vec!["a"])]);
        let different_messages = Bundle::new(1, vec![Message::with_strings("/bundle", vec!["c"])]);

        assert_eq!(lhs, rhs);
        assert_ne!(lhs, different_timetag);
        assert_ne!(lhs, different_messages);
    }
}
