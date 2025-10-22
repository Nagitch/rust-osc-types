# Changelog

## [0.1.0-alpha.2] - 2025-10-22
### Added
- Bundle nesting support - bundles can now contain other bundles in addition to messages
- `OscPacket` enum to unify handling of messages and bundles
- New Bundle methods: `empty()`, `add_message()`, `add_bundle()`
- Comprehensive test coverage for nested bundle scenarios

### Changed
- Bundle structure changed from `messages: Vec<Message>` to `packets: Vec<OscPacket>`
- Added convenience method `Bundle::with_messages()` for backward compatibility

## [0.1.0-alpha.1] - 2025-10-10
### Added
- Initial release.
