# Copilot Instructions for rust-osc-types

## Architecture Overview

This is a **multi-crate Rust workspace** implementing Open Sound Control (OSC) protocol types for different specification versions:
- `osc-types10/` - OSC 1.0 implementation (foundation crate)
- `osc-types11/` - OSC 1.1 implementation (extends osc-types10)
- Workspace root manages both crates with shared tooling

**Key Architecture Decisions:**
- Version-specific crates allow independent evolution of OSC spec implementations
- `osc-types11` depends on and extends `osc-types10` via `pub use`
- Both crates support `no_std` via conditional compilation

## Development Patterns

### Feature Flag Architecture
```rust
// Pattern used in both crates:
#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
```
- Default features: `["std"]`
- `no_std` support via `alloc` crate
- `unstable` feature for experimental APIs

### Code Standards
- Zero unsafe code policy (enforced with forbid attribute)
- Strict documentation requirements (enforced with deny attributes) 
- All public APIs must be documented

### Versioning Strategy
- **Experimental phase:** `0.1.0-alpha.x` versions
- **Breaking changes allowed** in pre-1.0 releases
- Exact version pinning between workspace crates (`=0.1.0-alpha.1`)
- Git tags follow pattern: `osc-types10-v*`, `osc-types11-v*`

## Workflow Commands

```bash
# Workspace-wide operations
cargo check --all                    # Check all crates
cargo test --all --all-features      # Test with all features
cargo clippy --all --all-features    # Lint all crates
cargo fmt --all -- --check          # Format check

# Individual crate operations
cd osc-types10 && cargo publish --no-verify
cd osc-types11 && cargo publish --no-verify
```

## Crate Dependencies
- `osc-types11` **must** depend on exact version of `osc-types10`
- Workspace uses `[patch.crates-io]` for local development
- MSRV: Rust 1.70+ for all crates

## Adding New OSC Types
1. Implement in appropriate version crate (`osc-types10` for baseline, `osc-types11` for extensions)
2. Follow lifetime parameter patterns: `Message<'a>` for borrowed data
3. Provide both `std` and `no_std` implementations using feature flags
4. Add comprehensive documentation with examples in rustdoc

## CI/Publishing
- CI runs on all pushes to `main` and PRs
- Individual crate publishing triggered by version-specific git tags
- No verification step in publish workflow (uses `--no-verify`)