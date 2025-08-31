#![allow(non_snake_case)]

use core::arch::naked_asm;

mod sqrmod;
pub(crate) use sqrmod::P256_sqrmod;

mod add_sub;
pub(crate) use add_sub::{P256_addmod, P256_submod};

pub(crate) mod montgomery;
use montgomery::Montgomery;

mod mulmod;
pub(self) use mulmod::P256_mulmod;

mod verify;
pub use verify::P256_verify_last_step;

/// The order of the P256 curve.
///
/// Code that relies on this static being fewer than 4096 bytes away
/// must be in the same `.p256-cortex-m4` section.
#[unsafe(link_section = ".p256-cortex-m4")]
pub(crate) static P256_ORDER: [u32; 9] = [
    0xFC632551, 0xF3B9CAC2, 0xA7179E84, 0xBCE6FAAD, 0xFFFFFFFF, 0xFFFFFFFF, 0, 0xFFFFFFFF, 0,
];

/// Code that relies on this static being fewer than 4096 bytes away
/// must be in the same `.p256-cortex-m4` section.
#[unsafe(link_section = ".p256-cortex-m4")]
pub(crate) static P256_B: [u32; 8] = [
    0x29c4bddf, 0xd89cdf62, 0x78843090, 0xacf005cd, 0xf7212ed6, 0xe5a220ab, 0x04874834, 0xdc30061d,
];

/// Code that relies on this static being fewer than 4096 bytes away
/// must be in the same `.p256-cortex-m4` section.
#[unsafe(link_section = ".p256-cortex-m4")]
pub(crate) static P256_PRIME: [u32; 8] =
    [0xffffffff, 0xffffffff, 0xffffffff, 0, 0, 0, 1, 0xffffffff];

// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
unsafe extern "C" {
    fn P256_negate_mod_m_if();

    fn P256_modinv_sqrt();
}

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

/// For inputs `A*R mod p` and `N`, computes the result of performing `A^2*R mod p` `N` times.
///
/// > **Note**: this function simply calls [`P256_sqrmod`] `N` times in an efficient manner.
///
/// # Inputs
/// Registers `r0` through `r7` shall contain `A*R mod p`.
///
/// Register `r8` shall contain `N`, the amount of times to perform the squaring operation.
///
/// # Return
/// On return the result of the operation will be contained in registers `r0` through `r7`.
///
/// All other registers are clobbered.
///
/// # Safety
/// The caller must ensure that the ABI for this function is upheld. It is impossible to do so from
/// normal rust code, so it must only be called from other inline assembly.
// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_sqrmod_many() {
    naked_asm!(
        "
            push {{r8, lr}}
            // frame push {{r8, lr}}
        0:
            bl {P256_sqrmod}

            ldr r8, [sp, #0]
            subs r8, r8, #1
            str r8, [sp, #0]
            bne 0b

            pop {{r8, pc}}
        ",
        P256_sqrmod = sym P256_sqrmod,
    )
}

/// If the inputs are `A*R mod p` and `B*R mod p`, calculate the result
/// of performing the operation `A^2*R mod p` `N` times, and multiplying that value by `B*R mod p`.
///
/// # Inputs
/// `r0` through `r7` shall contain `A*R mod p`.
///
/// `r8` shall contain `N`.
///
/// `r9` shall contain a valid `*const [u32; 8]`, whose dereference is `B*R mod p`.
///
/// # Return
/// `r0` through `r7` will contain the result of the computation.
///
/// All other registers are clobbered.
///
/// # Safety
/// The caller must ensure that the ABI for this function is upheld. It is impossible to do so from
/// normal rust code, so it must only be called from other inline assembly.
// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_sqrmod_many_and_mulmod() {
    naked_asm!(
        "
            push {{r9, lr}}
            // frame push {{r9, lr}}
            bl {P256_sqrmod_many}
            push {{r0-r7}}
            // frame address sp, 40
            mov r1, sp
            ldr r2, [sp,#32]
            bl {P256_mulmod}
            add sp, #36
            // frame address sp, 4
            pop {{pc}}
        ",
        P256_sqrmod_many = sym P256_sqrmod_many,
        P256_mulmod = sym P256_mulmod,
    )
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".p256-cortex-m4")]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_decompress_point(
    y: *const Montgomery,
    x: *const Montgomery,
    parity: u32,
) -> bool {
    naked_asm!(
        "
            push {{r0, r2, r4-r11, lr}}
            // frame push {{r0, r2, r4-r11, lr}}
            // frame address sp,44
            sub sp, #32
            // frame address sp, 76

            mov r0,sp
            bl {P256_to_montgomery}
            ldm sp, {{r0-r7}}

            bl {P256_sqrmod}
            push {{r0-r7}}

            mov r1, sp
            adr r2, {P256_B}
            bl {P256_submod}
            stm sp, {{r0-r7}}
            // frame address sp, 108

            add r1, sp, #32
            mov r2, sp
            bl {P256_mulmod}
            stm sp, {{r0-r7}}

            mov r1, sp
            adr r2, {P256_B}
            bl {P256_addmod}
            stm sp, {{r0-r7}}

            mov r8, #1
            bl {P256_modinv_sqrt}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            bl {P256_sqrmod}

            pop {{r8-r11}}
            // frame address sp, 92
            eors r8, r0
            ittt eq
            eorseq r9, r1
            eorseq r10, r2
            eorseq r11, r3
            pop {{r8-r11}}
            // frame address sp, 76
            itttt eq
            eorseq r8, r4
            eorseq r9, r5
            eorseq r10, r6
            eorseq r11, r7
            it ne
            movne r0, #0
            bne 0f

            mov r0, sp
            mov r1, sp
            bl {P256_from_montgomery}

            ldr r3, [sp]
            ldrd r0, r1, [sp, #32]
            and r2, r3, #1
            eors r2, r1
            mov r1, sp
            adr r3, {P256_PRIME}
            bl {P256_negate_mod_m_if}
            movs r0, #1
        0:
            add sp, #32 + 8
            // frame address sp, 36
            pop {{r4-r11, pc}}
        ",
        P256_to_montgomery = sym crate::P256_to_montgomery,
        P256_from_montgomery = sym crate::P256_from_montgomery,
        P256_sqrmod = sym P256_sqrmod,
        P256_mulmod = sym P256_mulmod,
        P256_submod = sym P256_submod,
        P256_addmod = sym P256_addmod,
        P256_negate_mod_m_if = sym P256_negate_mod_m_if,
        P256_modinv_sqrt = sym P256_modinv_sqrt,
        P256_B = sym P256_B,
        P256_PRIME = sym P256_PRIME,
    )
}
