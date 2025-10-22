#![forbid(unsafe_code)]
#![deny(missing_docs, unreachable_pub, rust_2018_idioms)]

//! # osc-types11
//! Extends osc-types10 for Open Sound Control 1.1.
//!
//! **Experimental / Not for production use**.

pub use osc_types10::{Bundle, Message, OscPacket, OscType};

#[cfg(test)]
mod tests {
    use super::{Bundle, Message, OscPacket, OscType};

    #[test]
    fn re_exports_message_type() {
        let msg = Message::with_strings("/re-export", vec!["arg"]);

        assert_eq!(msg.address, "/re-export");
        assert_eq!(msg.args, vec![OscType::String("arg")]);
    }

    #[test]
    fn re_exports_bundle_type() {
        let bundle =
            Bundle::with_messages(123, vec![Message::with_strings("/re-export", vec!["arg"])]);

        assert_eq!(bundle.timetag, 123);
        assert_eq!(bundle.packets.len(), 1);
        if let OscPacket::Message(ref msg) = bundle.packets[0] {
            assert_eq!(msg.address, "/re-export");
        } else {
            panic!("Expected message in bundle");
        }
    }
}
