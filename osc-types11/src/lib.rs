#![forbid(unsafe_code)]
#![deny(missing_docs, unreachable_pub, rust_2018_idioms)]

//! # osc-types11
//! Extends osc-types10 for Open Sound Control 1.1.
//!
//! **Experimental / Not for production use**.

pub use osc_types10::{Bundle, Message};

#[cfg(test)]
mod tests {
    use super::{Bundle, Message};

    #[test]
    fn re_exports_message_type() {
        let msg = Message::new("/re-export", vec!["arg"]);

        assert_eq!(msg.address, "/re-export");
        assert_eq!(msg.args, vec!["arg"]);
    }

    #[test]
    fn re_exports_bundle_type() {
        let bundle = Bundle::new(123, vec![Message::new("/re-export", vec!["arg"])]);

        assert_eq!(bundle.timetag, 123);
        assert_eq!(bundle.messages.len(), 1);
        assert_eq!(bundle.messages[0].address, "/re-export");
    }
}
