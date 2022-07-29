# P256-CM4

[![CI](https://github.com/newAM/p256-cm4/workflows/CI/badge.svg)](https://github.com/newAM/p256-cm4/actions)

A (mostly) rust re-implementation of [Emill/P256-Cortex-M4].

Rust 1.59.0 stabilized the [global_asm] macro which allows for this to be compiled without any additional tooling.  No `build.rs` script or external assembler required.

This is in a pre-alpha state, it requires more testing for ECDH, and it is not published to crates.io.  See [ycrypto/p256-cortex-m4] for a complete implementation.

## Comparisons

TODO: Size and speed comparisons with the STM32 public-key accelerator and rust-crypto's p256.

## Maintainers Notes

### Testing

Install [probe-run].

Adjust `.cargo/config.toml` and `memory.x` for your target.

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
[probe-run]: https://github.com/knurling-rs/probe-run
[ycrypto/p256-cortex-m4]: https://github.com/ycrypto/p256-cortex-m4
