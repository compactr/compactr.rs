# Changelog

## [0.1.0] Initial release

### Added

- Initial project structure with workspace configuration
- Core error types using `thiserror`
- `Value` enum for runtime value representation
- Schema system with `SchemaType` and `Property` definitions
- Thread-safe `SchemaRegistry` with reference resolution
- Circular reference detection in schema registry
- Complete encoder implementation for all schema types
- Complete decoder implementation for all schema types
- Buffer utilities for string and binary encoding/decoding
- UUID format support (16-byte compact encoding)
- DateTime format support (8-byte Unix timestamp in milliseconds)
- Date format support (4-byte days since epoch)
- IPv4 address support (4-byte encoding)
- IPv6 address support (16-byte encoding)
- Binary data support with length prefix
- Array encoding/decoding with length prefix
- Object encoding/decoding with schema-ordered properties
- Comprehensive unit tests for all formats
- Integration tests covering all major use cases
- Basic usage example
- Advanced example with all format types
- Project documentation (README, CONTRIBUTING, ARCHITECTURE)
- GitHub Actions CI/CD workflow
- Rustfmt and Clippy configuration
- Dual MIT/Apache-2.0 licensing

