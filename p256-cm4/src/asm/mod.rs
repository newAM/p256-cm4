#![allow(non_snake_case)]

use core::arch::naked_asm;

mod add_sub;
pub(crate) use add_sub::{P256_addmod, P256_submod};

pub(crate) mod montgomery;
use montgomery::Montgomery;

// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
unsafe extern "C" {
    /// For inputs `A*R mod p` and `B*R mod p`, compute `A*B*R mod p`.
    ///
    /// # Inputs
    /// Register `r0` shall contain a valid `*const [u32; 8]`. TODO: figure out ordering. Montgomery?
    ///
    /// Register `r1` shall contain a valid `*const [u32; 8]`. TODO: figure out ordering. Montgomery?
    ///
    /// # Return
    /// On return `r0` through `r7` contains `A*B*R mod p`.
    ///
    /// All other registers are clobbered.
    fn P256_mulmod();

    /// If input is `A*R mod p`, compute `A^2*R mod p`.
    ///
    /// # Inputs
    /// Registers `r0` through `r7` shall contain `A*R mod p`. TODO: figure out ordering. Montgomery?
    ///
    /// # Return
    /// On return `A^2*R mod p` will be contained in `r0` through `r7`.
    ///
    /// All other registers are clobbered.
    fn P256_sqrmod();
}

/// Code that relies on this static being fewer than 4096 bytes away
/// must be in the same `.p256-cortex-m4` section.
#[unsafe(link_section = ".p256-cortex-m4")]
pub(crate) static P256_B: [u32; 8] = [
    0x29c4bddf, 0xd89cdf62, 0x78843090, 0xacf005cd, 0xf7212ed6, 0xe5a220ab, 0x04874834, 0xdc30061d,
];

/// Check if a point `xy` is on the `p256` curve.
///
/// # Inputs
/// `r1` shall contain `x`, a valid [`*const Montgomery`](`Montgomery`).
///
/// `r2` shall contain `y`, a valid [`*const Montgomery`](`Montgomery`).
///
/// # Return
/// On return, `r0` will contain `1` if `xy` is on the `p256` curve. Otherwise, `r0` will contain `0`.
///
/// All other registers are clobbered.
///
/// # Safety
/// The caller must ensure that the ABI for this function is upheld. It is impossible to do so from
/// normal rust code, so it must only be called from other inline assembly.
// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
#[unsafe(no_mangle)]
#[unsafe(link_section = ".p256-cortex-m4")]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_point_is_on_curve(
    x: *const Montgomery,
    y: *const Montgomery,
) -> bool {
    naked_asm!(
        "
            push {{r0, r4-r11, lr}}
            // frame push {{r0, r4-r11, lr}}
            // frame address sp, 40

            // We verify y^2 - x(x^2 - 3) = b

            // y^2
            ldm r1, {{r0-r7}}
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp, 72

            // x^2
            ldr r0, [sp,#32]
            ldm r0, {{r0-r7}}
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp, 104

            // x^2 - 3
            mov r1, sp
            adr r2, 1f
            bl {P256_submod}
            stm sp, {{r0-r7}}

            // x(x^2 - 3)
            ldr r1, [sp,#64]
            mov r2, sp
            bl {P256_mulmod}
            stm sp, {{r0-r7}}

            // y^2 - x(x^2 - 3)
            add r1, sp, #32
            mov r2, sp
            bl {P256_submod}

            // compare with b
            adr r8, {P256_B}
            ldm r8!, {{r9-r12}}
            eors r0, r9
            ittt eq
            eorseq r1, r10
            eorseq r2, r11
            eorseq r3, r12
            ldm r8, {{r9-r12}}
            itttt eq
            eorseq r4, r9
            eorseq r5, r10
            eorseq r6, r11
            eorseq r7, r12
            mov r0, #0
            it eq
            moveq r0, #1

            add sp, #68
            //frame address sp,36

            pop {{r4-r11,pc}}

            .align 2
        1: // three_mont, the Montgomery representation of the number `3`.
            .word 0x3
            .word 0x0
            .word 0x0
            .word 0xfffffffd
            .word 0xffffffff
            .word 0xffffffff
            .word 0xfffffffc
            .word 0x2
        ",
        P256_mulmod = sym P256_mulmod,
        P256_submod = sym P256_submod,
        P256_sqrmod = sym P256_sqrmod,
        P256_B = sym P256_B,
    )
}
