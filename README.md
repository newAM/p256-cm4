# P256-CM4

[![CI](https://github.com/newAM/p256-cm4/workflows/CI/badge.svg)](https://github.com/newAM/p256-cm4/actions)
[![crates.io](https://img.shields.io/crates/v/p256-cm4.svg)](https://crates.io/crates/p256-cm4)
[![docs](https://docs.rs/p256-cm4/badge.svg)](https://docs.rs/p256-cm4) 

A (mostly) rust re-implementation of [Emill/P256-Cortex-M4].

Rust 1.59.0 stabilized the [global_asm] macro which allows for this to be compiled without any additional tooling.  No `build.rs` script or external assembler required.

## Limitations

This is not yet complete, it lacks safe wrappers for the `unsafe` functions.  See [ycrypto/p256-cortex-m4] for a safe implementation.

This lacks the configurability of the original source because rust features are less powerful than C pre-processor macros.  Use [ycrypto/p256-cortex-m4-sys] if you require configurability.

## Comparisons

As measured on a STM32WLE5.

| Implementation | Signing Cycles (appx) | Verify Cycles (appx) | Flash Size (appx) |
|----------------|-----------------------|----------------------|-------------------|
| Hardware PKA   |             5,211,859 |           10,516,860 |           1,582 B |
| [RustCrypto]   |             7,856,436 |           14,303,277 |            49 kiB |
| `p256-cm4`     |               442,754 |            1,225,746 |            10 kiB |

## Maintainers Notes

### Testing

Install [probe-run].

Adjust `.cargo/config.toml`, `memory.x`, `testsuite/Cargo.toml`, and the clock setup for your target.

```bash
cargo test -p testsuite
```

### ASM Generation

Send the GCC ASM from [Emill/P256-Cortex-M4] through the pre-processor.

```bash
arm-none-eabi-gcc -O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -mthumb -march=armv7e-m -Wall -Wextra -std=c11 -march=armv7e-m -c P256-Cortex-M4/p256-cortex-m4-asm-gcc.S -E > asm.s
```

[Emill/P256-Cortex-M4]: https://github.com/Emill/P256-Cortex-M4
[global_asm]: https://doc.rust-lang.org/core/arch/macro.global_asm.html
[ycrypto/p256-cortex-m4]: https://github.com/ycrypto/p256-cortex-m4
[ycrypto/p256-cortex-m4-sys]: https://github.com/ycrypto/p256-cortex-m4-sys
[RustCrypto]: https://github.com/RustCrypto/elliptic-curves
[probe-run]: https://github.com/knurling-rs/probe-run
