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
use alloc::vec::Vec;

/// OSC packet - either a message or a bundle
#[derive(Debug, Clone, PartialEq)]
pub enum OscPacket<'a> {
    /// An OSC message
    Message(Message<'a>),
    /// An OSC bundle
    Bundle(Bundle<'a>),
}

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
    /// Arguments of the message
    pub args: Vec<OscType<'a>>,
}

impl<'a> Message<'a> {
    /// Create a new OSC message
    pub fn new(address: &'a str, args: Vec<OscType<'a>>) -> Self {
        Self { address, args }
    }

    /// Create a new OSC message with string arguments (convenience method)
    pub fn with_strings(address: &'a str, string_args: Vec<&'a str>) -> Self {
        let args = string_args.into_iter().map(OscType::String).collect();
        Self::new(address, args)
    }
}

/// OSC Bundle as defined in OSC 1.0 specification
///
/// Bundles can contain both messages and nested bundles, allowing for hierarchical organization
/// of OSC data with precise timing control.
#[derive(Debug, Clone, PartialEq)]
pub struct Bundle<'a> {
    /// OSC time tag (64-bit NTP timestamp)
    pub timetag: u64,
    /// Packets contained in the bundle (messages and/or nested bundles)
    pub packets: Vec<OscPacket<'a>>,
}

impl<'a> Bundle<'a> {
    /// Create a new OSC bundle
    pub fn new(timetag: u64, packets: Vec<OscPacket<'a>>) -> Self {
        Self { timetag, packets }
    }

    /// Create a new OSC bundle with only messages (convenience method)
    pub fn with_messages(timetag: u64, messages: Vec<Message<'a>>) -> Self {
        let packets = messages.into_iter().map(OscPacket::Message).collect();
        Self::new(timetag, packets)
    }

    /// Create a new empty bundle
    pub fn empty(timetag: u64) -> Self {
        Self::new(timetag, Vec::new())
    }

    /// Add a message to the bundle
    pub fn add_message(&mut self, message: Message<'a>) {
        self.packets.push(OscPacket::Message(message));
    }

    /// Add a nested bundle to the bundle
    pub fn add_bundle(&mut self, bundle: Bundle<'a>) {
        self.packets.push(OscPacket::Bundle(bundle));
    }
}

#[cfg(test)]
mod tests {
    use super::{Bundle, Message, OscPacket, OscType};

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
        assert_eq!(msg.args.len(), 4);
    }

    #[test]
    fn bundle_new_sets_timetag_and_packets() {
        let messages = vec![
            Message::with_strings("/bundle/one", vec!["1"]),
            Message::with_strings("/bundle/two", vec!["2"]),
        ];
        let bundle = Bundle::with_messages(42, messages.clone());

        assert_eq!(bundle.timetag, 42);
        // Check that packets contain the expected messages
        assert_eq!(bundle.packets.len(), 2);
        if let OscPacket::Message(ref msg) = bundle.packets[0] {
            assert_eq!(msg, &messages[0]);
        } else {
            panic!("Expected message in bundle");
        }
    }

    #[test]
    fn bundle_equality_compares_contents() {
        let messages = vec![
            Message::with_strings("/bundle", vec!["a"]),
            Message::with_strings("/bundle", vec!["b"]),
        ];
        let lhs = Bundle::with_messages(1, messages.clone());
        let rhs = Bundle::with_messages(1, messages);
        let different_timetag =
            Bundle::with_messages(2, vec![Message::with_strings("/bundle", vec!["a"])]);
        let different_messages =
            Bundle::with_messages(1, vec![Message::with_strings("/bundle", vec!["c"])]);

        assert_eq!(lhs, rhs);
        assert_ne!(lhs, different_timetag);
        assert_ne!(lhs, different_messages);
    }

    #[test]
    fn bundle_supports_nested_bundles() {
        let inner_bundle = Bundle::with_messages(
            100,
            vec![Message::with_strings("/inner/msg", vec!["inner"])],
        );
        let mut outer_bundle = Bundle::empty(200);
        outer_bundle.add_message(Message::with_strings("/outer/msg", vec!["outer"]));
        outer_bundle.add_bundle(inner_bundle.clone());

        assert_eq!(outer_bundle.timetag, 200);
        assert_eq!(outer_bundle.packets.len(), 2);

        // Check the outer message
        if let OscPacket::Message(ref msg) = outer_bundle.packets[0] {
            assert_eq!(msg.address, "/outer/msg");
        } else {
            panic!("Expected message at index 0");
        }

        // Check the nested bundle
        if let OscPacket::Bundle(ref bundle) = outer_bundle.packets[1] {
            assert_eq!(bundle, &inner_bundle);
        } else {
            panic!("Expected bundle at index 1");
        }
    }

    #[test]
    fn bundle_add_methods_work_correctly() {
        let mut bundle = Bundle::empty(42);

        let msg1 = Message::with_strings("/test1", vec!["a"]);
        let msg2 = Message::with_strings("/test2", vec!["b"]);
        let nested_bundle = Bundle::with_messages(100, vec![msg2.clone()]);

        bundle.add_message(msg1.clone());
        bundle.add_bundle(nested_bundle.clone());

        assert_eq!(bundle.packets.len(), 2);

        // Verify message was added correctly
        if let OscPacket::Message(ref msg) = bundle.packets[0] {
            assert_eq!(msg, &msg1);
        } else {
            panic!("Expected message at index 0");
        }

        // Verify bundle was added correctly
        if let OscPacket::Bundle(ref b) = bundle.packets[1] {
            assert_eq!(b, &nested_bundle);
        } else {
            panic!("Expected bundle at index 1");
        }
    }

    #[test]
    fn bundle_with_messages_convenience_method() {
        let messages = vec![
            Message::with_strings("/msg1", vec!["hello"]),
            Message::with_strings("/msg2", vec!["world"]),
        ];

        let bundle = Bundle::with_messages(42, messages.clone());

        assert_eq!(bundle.timetag, 42);
        assert_eq!(bundle.packets.len(), 2);

        for (i, expected_msg) in messages.iter().enumerate() {
            if let OscPacket::Message(ref msg) = bundle.packets[i] {
                assert_eq!(msg, expected_msg);
            } else {
                panic!("Expected message at index {}", i);
            }
        }
    }
}
