#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeTag(pub u64);

impl TimeTag {
    pub const IMMEDIATE: TimeTag = TimeTag(1);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Address(pub String);

impl Address {
    pub fn is_plausible(&self) -> bool {
        !self.0.is_empty() && self.0.as_bytes()[0] == b'/'
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OscType {
    Int(i32),
    Float(f32),
    String(String),
    Blob(Vec<u8>),

    #[cfg(feature = "osc-1-1")]
    Int64(i64),
    #[cfg(feature = "osc-1-1")]
    Double(f64),
    #[cfg(feature = "osc-1-1")]
    TimeTag(TimeTag),
    #[cfg(feature = "osc-1-1")]
    True,
    #[cfg(feature = "osc-1-1")]
    False,
    #[cfg(feature = "osc-1-1")]
    Nil,
    #[cfg(feature = "osc-1-1")]
    Infinitum,
    #[cfg(feature = "osc-1-1")]
    Midi(u32),
    #[cfg(feature = "osc-1-1")]
    Color(u32),
    #[cfg(feature = "osc-1-1")]
    Char(char),
    #[cfg(feature = "osc-1-1")]
    Array(Vec<OscType>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OscMessage {
    pub addr: Address,
    pub args: Vec<OscType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OscBundle {
    pub timetag: TimeTag,
    pub elements: Vec<OscPacket>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OscPacket {
    Message(OscMessage),
    Bundle(OscBundle),
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn address_plausibility_requires_leading_slash() {
        let plausible = Address("/synth/volume".into());
        let implausible = Address("synth/volume".into());

        assert!(plausible.is_plausible());
        assert!(!implausible.is_plausible());
    }

    #[test]
    fn osc_message_supports_core_types() {
        let message = OscMessage {
            addr: Address("/core".into()),
            args: vec![
                OscType::Int(42),
                OscType::Float(0.5),
                OscType::String("hi".into()),
                OscType::Blob(vec![1, 2, 3, 4]),
            ],
        };

        assert_eq!(message.addr, Address("/core".into()));
        assert_eq!(message.args.len(), 4);
        assert_eq!(message.args[0], OscType::Int(42));
        assert_eq!(message.args[1], OscType::Float(0.5));
        assert_eq!(message.args[2], OscType::String("hi".into()));
        assert_eq!(message.args[3], OscType::Blob(vec![1, 2, 3, 4]));
    }

    #[test]
    fn osc_bundle_supports_nested_packets() {
        let inner_message = OscMessage {
            addr: Address("/inner".into()),
            args: vec![OscType::Int(1)],
        };
        let packet = OscPacket::Message(inner_message.clone());
        let bundle = OscBundle {
            timetag: TimeTag::IMMEDIATE,
            elements: vec![packet.clone()],
        };

        assert_eq!(bundle.timetag, TimeTag::IMMEDIATE);
        assert_eq!(bundle.elements.len(), 1);
        assert_eq!(bundle.elements[0], packet);
        if let OscPacket::Message(message) = &bundle.elements[0] {
            assert_eq!(message, &inner_message);
        } else {
            panic!("expected message packet");
        }
    }

    #[cfg(feature = "osc-1-1")]
    #[test]
    fn osc_1_1_specific_types_are_supported() {
        let array_arg = OscType::Array(vec![
            OscType::Int64(i64::MAX),
            OscType::Double(core::f64::consts::PI),
            OscType::True,
            OscType::False,
            OscType::Nil,
            OscType::Infinitum,
            OscType::Midi(0x1234_5678),
            OscType::Color(0xff00_00ff),
            OscType::Char('a'),
        ]);

        let message = OscMessage {
            addr: Address("/extended".into()),
            args: vec![
                OscType::TimeTag(TimeTag(10)),
                array_arg.clone(),
            ],
        };

        assert_eq!(message.args.len(), 2);
        assert_eq!(message.args[0], OscType::TimeTag(TimeTag(10)));
        assert_eq!(message.args[1], array_arg);
    }
}
