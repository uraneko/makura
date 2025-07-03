# CHANGELOG

## [Unreleased]

### Added 
- makura_cli crate at version 0.1.0

- decoder unit tests
- makura lib.rs unit tests

- ci run cargo doc 
- ci run cargo clippy 
- ci run cargo test --all-features

### Updated
- makura crate version to 0.1.2

### Changed
- encoding and decoding implementations

### Removed
- the force_decode decoding function

### Fixed 
- buggy decoding logic

## [0.1.1]

### Added 
- base45 encoding/decoding logic
- integration tests for all bases 

### Updated
- makura crate version to 0.1.1

### Changed
- encoding and decoding implementations

### Removed
- panicing behavior from decode functions

### Fixed 
- buggy decoding logic

## [0.1.0]

### Added 
- makura crate at version to 0.1.0

- base 64(url) encoding/decoding
- base 32(hex) encoding/decoding
- base 16 encoding/decoding
