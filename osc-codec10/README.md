# osc-codec10

Minimal, zero-copy-leaning OSC 1.0 encoder/decoder that pairs with [`osc-types10`](https://github.com/Nagitch/rust-osc-types).

## Status

Experimental preview for integration testing with `osc-types10`. API may break.

## Install

Add to your workspace and depend on the Git dependency:

```toml
[dependencies]
osc-codec10 = { path = "../osc-codec10" } # if local
# or, if published later:
# osc-codec10 = "0.1"
```

This crate depends on `osc-types10` via the GitHub repo. If you vendored or published it, adjust `Cargo.toml` accordingly.

## no_std

Default feature set uses `std`. For `no_std + alloc`:

```bash
cargo build -p osc-codec10 --no-default-features --features alloc
```

## Usage

```rust
use osc_types10::{Message, OscType, Bundle};
use osc_codecs10 as _; // refer to actual crate name
```

See `examples/` for UDP send/recv.

## License

Dual-licensed under either of

- MIT License (LICENSE-MIT)
- Apache License, Version 2.0 (LICENSE-APACHE)
