//! osc-codec10: a small, no_std-friendly OSC 1.0 encoder/decoder
//!
//! - Zero-copy leaning: decoded Strings/Blobs borrow from the input buffer.
//! - Strict 4-byte OSC alignment for strings/blobs.
//! - Big endian numeric encoding per the OSC 1.0 spec.
//! - Minimal scope: Messages and Bundles (bundle contains only messages in this first cut).
//!
//! ## no_std
//! Default builds use `std`. For `no_std + alloc`:
//! ```shell
//! cargo build -p osc-codec10 --no-default-features --features alloc
//! ```
//!
//! ## Examples
//! See `examples/` for UDP send/recv samples (require `std`).

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use byteorder::{BigEndian, ByteOrder};
use core::str;
use osc_types10::{Bundle, Message, OscPacket, OscType};

/// Errors that can occur while decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Truncated,
    InvalidString,
    InvalidTag,
    UnexpectedEof,
    /// Error for malformed bundle elements (deprecated - bundles can now contain both messages and bundles).
    NonMessageInBundle,
}

pub type Result<T> = core::result::Result<T, Error>;

#[inline]
fn pad4_len(len: usize) -> usize {
    (4 - (len & 3)) & 3
}

fn put_str(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);
    let pad = pad4_len(s.len() + 1);
    buf.extend(std::iter::repeat_n(0, pad));
}

fn get_cstr_4(bytes: &[u8], mut off: usize) -> Result<(&str, usize)> {
    // Find NUL terminator
    let start = off;
    while off < bytes.len() && bytes[off] != 0 {
        off += 1;
    }
    if off >= bytes.len() {
        return Err(Error::Truncated);
    }
    let s = core::str::from_utf8(&bytes[start..off]).map_err(|_| Error::InvalidString)?;
    off += 1; // skip NUL
              // Skip padding to 4-byte boundary
    let pad = pad4_len(off - start);
    if off + pad > bytes.len() {
        return Err(Error::Truncated);
    }
    Ok((s, off + pad))
}

#[inline]
fn put_i32(buf: &mut Vec<u8>, v: i32) {
    let mut tmp = [0u8; 4];
    BigEndian::write_i32(&mut tmp, v);
    buf.extend_from_slice(&tmp);
}
#[inline]
fn put_f32(buf: &mut Vec<u8>, v: f32) {
    let mut tmp = [0u8; 4];
    BigEndian::write_f32(&mut tmp, v);
    buf.extend_from_slice(&tmp);
}

#[inline]
fn get_i32(bytes: &[u8], off: &mut usize) -> Result<i32> {
    if *off + 4 > bytes.len() {
        return Err(Error::UnexpectedEof);
    }
    let v = BigEndian::read_i32(&bytes[*off..*off + 4]);
    *off += 4;
    Ok(v)
}
#[inline]
fn get_f32(bytes: &[u8], off: &mut usize) -> Result<f32> {
    if *off + 4 > bytes.len() {
        return Err(Error::UnexpectedEof);
    }
    let v = BigEndian::read_f32(&bytes[*off..*off + 4]);
    *off += 4;
    Ok(v)
}

/// Encode a single OSC message into bytes.
pub fn encode_message(msg: &Message<'_>) -> Vec<u8> {
    let mut buf = Vec::new();
    put_str(&mut buf, msg.address);

    // Type tag (starts with ',')
    let mut tag = String::from(",");
    for a in &msg.args {
        match a {
            OscType::Int(_) => tag.push('i'),
            OscType::Float(_) => tag.push('f'),
            OscType::String(_) => tag.push('s'),
            OscType::Blob(_) => tag.push('b'),
        }
    }
    put_str(&mut buf, &tag);

    for a in &msg.args {
        match a {
            OscType::Int(v) => put_i32(&mut buf, *v),
            OscType::Float(v) => put_f32(&mut buf, *v),
            OscType::String(s) => put_str(&mut buf, s),
            OscType::Blob(b) => {
                put_i32(&mut buf, b.len() as i32);
                buf.extend_from_slice(b);
                let pad = pad4_len(b.len());
                buf.extend(std::iter::repeat_n(0, pad));
            }
        }
    }
    buf
}

/// Decode a single OSC message from bytes, returning the message and number of bytes consumed.
pub fn decode_message<'a>(bytes: &'a [u8]) -> Result<(Message<'a>, usize)> {
    let (address, mut off) = get_cstr_4(bytes, 0)?;
    let (tag, off2) = get_cstr_4(bytes, off)?;
    off = off2;

    let mut args = Vec::new();
    let mut chars = tag.chars();
    if chars.next() != Some(',') {
        return Err(Error::InvalidTag);
    }

    for t in chars {
        match t {
            'i' => {
                args.push(OscType::Int(get_i32(bytes, &mut off)?));
            }
            'f' => {
                args.push(OscType::Float(get_f32(bytes, &mut off)?));
            }
            's' => {
                let (s, new_off) = get_cstr_4(bytes, off)?;
                args.push(OscType::String(s));
                off = new_off;
            }
            'b' => {
                let len = get_i32(bytes, &mut off)? as usize;
                if off + len > bytes.len() {
                    return Err(Error::UnexpectedEof);
                }
                let blob = &bytes[off..off + len];
                off += len;
                let pad = pad4_len(len);
                if off + pad > bytes.len() {
                    return Err(Error::UnexpectedEof);
                }
                off += pad;
                args.push(OscType::Blob(blob));
            }
            _ => return Err(Error::InvalidTag),
        }
    }

    Ok((Message::new(address, args), off))
}

const BUNDLE_TAG: &str = "#bundle";

/// Encode a bundle that can contain messages and nested bundles.
pub fn encode_bundle(b: &Bundle<'_>) -> Vec<u8> {
    let mut buf = Vec::new();
    put_str(&mut buf, BUNDLE_TAG);
    // 64-bit big-endian NTP timetag
    let mut tt = [0u8; 8];
    BigEndian::write_u64(&mut tt, b.timetag);
    buf.extend_from_slice(&tt);

    for packet in &b.packets {
        let pkt = match packet {
            OscPacket::Message(msg) => encode_message(msg),
            OscPacket::Bundle(bundle) => encode_bundle(bundle),
        };
        put_i32(&mut buf, pkt.len() as i32);
        buf.extend_from_slice(&pkt);
    }
    buf
}

/// Decode a bundle that can contain messages and nested bundles. Returns the bundle and number of bytes consumed.
pub fn decode_bundle<'a>(bytes: &'a [u8]) -> Result<(Bundle<'a>, usize)> {
    let (tag, mut off) = get_cstr_4(bytes, 0)?;
    if tag != BUNDLE_TAG {
        return Err(Error::InvalidString);
    }
    if off + 8 > bytes.len() {
        return Err(Error::Truncated);
    }
    let timetag = BigEndian::read_u64(&bytes[off..off + 8]);
    off += 8;

    let mut packets = Vec::new();
    while off < bytes.len() {
        let size = get_i32(bytes, &mut off)? as usize;
        if off + size > bytes.len() {
            return Err(Error::Truncated);
        }

        let element_bytes = &bytes[off..off + size];

        // Try to determine if this is a bundle by checking if it has a valid bundle structure
        // A bundle must have at minimum: "#bundle\0" (8 bytes aligned) + 8-byte timetag = 16 bytes
        let is_bundle =
            if element_bytes.len() >= 16 && element_bytes.starts_with(BUNDLE_TAG.as_bytes()) {
                // Check if it's properly null-terminated and 4-byte aligned like a real bundle
                let tag_end = BUNDLE_TAG.len();
                element_bytes.get(tag_end) == Some(&0) && {
                    // Calculate where the timetag should start (after null-terminated "#bundle" + padding)
                    let tag_with_null_len = tag_end + 1;
                    let padding = pad4_len(tag_with_null_len);
                    let timetag_start = tag_with_null_len + padding;
                    // Ensure we have enough bytes for the timetag
                    element_bytes.len() >= timetag_start + 8
                }
            } else {
                false
            };

        if is_bundle {
            // Try to decode as bundle first, fall back to message if it fails
            match decode_bundle(element_bytes) {
                Ok((bundle, used)) if used == size => {
                    packets.push(OscPacket::Bundle(bundle));
                }
                _ => {
                    // Bundle decoding failed, treat as message
                    let (msg, used) = decode_message(element_bytes)?;
                    if used != size {
                        return Err(Error::InvalidTag);
                    }
                    packets.push(OscPacket::Message(msg));
                }
            }
        } else {
            let (msg, used) = decode_message(element_bytes)?;
            if used != size {
                return Err(Error::InvalidTag);
            }
            packets.push(OscPacket::Message(msg));
        }

        off += size;
    }
    Ok((Bundle::new(timetag, packets), off))
}
