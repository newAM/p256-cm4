# P256-CM4

[![CI](https://github.com/newAM/p256-cm4/workflows/CI/badge.svg)](https://github.com/newAM/p256-cm4/actions)
[![crates.io](https://img.shields.io/crates/v/p256-cm4.svg)](https://crates.io/crates/p256-cm4)
[![docs](https://docs.rs/p256-cm4/badge.svg)](https://docs.rs/p256-cm4)

A (mostly) rust re-implementation of [Emill/P256-Cortex-M4].

Rust 1.88.0 stabilized the [naked_asm] macro which allows for this to be compiled without any additional tooling.  No `build.rs` script or external assembler required.

## Limitations

This is not yet complete, it lacks interoperability with other targets (via RustCrypto traits or compile-time switches).  See [ycrypto/p256-cortex-m4] for an interoperable solution.

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

Install [qemu-system-arm] (tested to work with `qemu-system-arm 8.2.2`).

```bash
DEFMT_LOG=trace cargo test -p testsuite --target thumbv7em-none-eabi
```

### Testing on real hardware

1. Install a debugger (we recommend [probe-rs-tools])
2. Update `memory.x` to match your target device
3. Update the runner in `.cargo/config.toml`
4. Run the command from the [Testing](#testing) section.

Usually, you'll want rtt logs instead of semihosting logs when running on real hardware. To get those, you can enable the `rtt` feature.

An example change, when running on an `STM32H723ZGTx`:

`.cargo/config.toml`:
```diff
[target.thumbv7em-none-eabi]
-runner = "cargo run -p qemu-decode --"
+runner = "probe-rs run --chip STM32H723ZGTx"
```

`memory.x`:
```diff
-/* Memory for the LM3S6965EVB */
+/* Memory for STM32H723ZGTx (running from RAM) */
MEMORY
{
-  FLASH : ORIGIN = 0x00000000, LENGTH = 256k
-  RAM : ORIGIN = 0x20000000, LENGTH = 64k
+  DTCM    : ORIGIN = 0x20000000, LENGTH = 128K
+  AXISRAM : ORIGIN = 0x24000000, LENGTH = 128K + 192K
}
+# Region alias to run from RAM
+REGION_ALIAS(FLASH, AXISRAM);
+REGION_ALIAS(RAM,   DTCM);
```

Command:
```bash
DEFMT_LOG=trace cargo test -p testsuite --target thumbv7em-none-eabi --features rtt
```

[probe-rs-tools]: https://probe.rs/docs/getting-started/installation/
[Emill/P256-Cortex-M4]: https://github.com/Emill/P256-Cortex-M4
[naked_asm]: https://doc.rust-lang.org/core/arch/macro.naked_asm.html
[ycrypto/p256-cortex-m4]: https://github.com/ycrypto/p256-cortex-m4
[ycrypto/p256-cortex-m4-sys]: https://github.com/ycrypto/p256-cortex-m4-sys
[RustCrypto]: https://github.com/RustCrypto/elliptic-curves
[qemu-system-arm]: https://www.qemu.org/docs/master/system/target-arm.html
