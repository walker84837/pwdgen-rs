# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this
project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.4.0] - 2024-08-06

### Added

- Added a new `sections` option for password generation, allowing users
  to specify the number of sections in the generated password.

### Changed

- Changed the binary name to 'cwv' in documentation and PKGBUILD script.
- Rename project from 'pwdgen' to 'cryptweave'.

## [0.3.1] - 2024-04-17

### Changed

- Improve password generator function in `main.rs` for clarity.
- Update dependencies in `Cargo.lock`
- Update `Cargo.toml` to include licensing information, categories, keywords, and repository link.
- Tweak build settings in Cargo.toml:
    - enabling stripping,
    - link-time optimization (LTO)
    - overflow checks
    - setting panic to `abort`

## [0.3.0] - 2024-02-16

### Changed

  - Rewrite in Rust.

## [0.2.1] - 2023-12-16

### Fixed

  - Fixed a bug where the `get_random_int()` function would generate random
    numbers incorrectly.
  - The code style is now more consistent, using the Linux kernel coding style.

## [0.2.0] - 2023-11-27

### Removed

  - Removed the usage of C's `rand()` function.

### Changed

  - Replaced C's `rand()` function with OpenSSL's random function for security
    purposes.

## [0.1.0] - 2023-11-02

### Added

  - Initial release of `cpasswordgenerator`.
