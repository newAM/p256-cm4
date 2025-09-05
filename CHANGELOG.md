# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Updated the edition from 2021 to 2024.
- Change `global_asm` implementation to individual `naked_asm` functions ([#23](https://github.com/newAM/p256-cm4/pull/23)).

## [0.3.0] - 2022-08-07
### Changed
- Added safe wrapper functions for `P256_check_range_p` and `P256_check_range_n`.
  - Removed `P256_` prefix.
  - Removed `unsafe` requirement because these are no longer `extern "C"`.
  - Added `#[must_use]`.

## [0.2.0] - 2022-08-06
### Changed
- Changed all rust functions to use slices instead of pointers.

### Removed
- Removed `extern "C"` from all rust functions.
- Removed `#[no_mangle]` from all rust functions.
- Removed `unsafe` from all rust functions.
- Removed the `p256_` prefix from all rust functions.

## [0.1.1] - 2022-08-01
### Changed
- Added `#[cfg(target_arch = "arm")]` to the assembly to improve compatibility with IDE and CI tools.

## [0.1.0] - 2022-07-30
- Initial release

[Unreleased]: https://github.com/newAM/p256-cm4/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/newAM/p256-cm4/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/newAM/p256-cm4/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/newAM/p256-cm4/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/newAM/p256-cm4/releases/tag/v0.1.0
