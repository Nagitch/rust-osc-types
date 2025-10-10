# osc-types10

> **⚠ Experimental / Not recommended for production use**

[![Crates.io](https://img.shields.io/crates/v/osc-types10.svg)](https://crates.io/crates/osc-types10)
[![Docs.rs](https://docs.rs/osc-types10/badge.svg)](https://docs.rs/osc-types10)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue)

Implementation of **Open Sound Control 1.0** message and bundle types for Rust.
This crate is part of the [`rust-osc-types`](https://github.com/Nagitch/rust-osc-types) workspace.

## ⚠ Stability Notice

- Experimental and under development.
- Breaking changes may occur until 1.0.0.
- Intended for research and testing.

## Example

```toml
osc-types10 = "=0.1.0-alpha.1"
```

```rust
use osc_types10::Message;
let msg = Message::new("/example", vec!["test".into()]);
println!("{msg:?}");
```

## License

Dual licensed under MIT OR Apache-2.0.
