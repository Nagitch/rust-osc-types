#![forbid(unsafe_code)]
#![deny(missing_docs, unreachable_pub, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # osc-types10
//! **⚠ Experimental / Not for production use**
//!
//! Defines message and bundle types for Open Sound Control 1.0.
//! - Pre-release (`0.1.0-alpha`)
//! - Breaking changes may occur frequently
//! - `no_std` compatible (optional)
//!
//! ## Example
//! ```rust
//! use osc_types10::Message;
//! let msg = Message::new("/example", vec!["abc".into()]);
//! println!("{msg:?}");
//! ```

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
