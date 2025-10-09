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
