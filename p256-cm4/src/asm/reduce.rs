use core::arch::naked_asm;

use crate::sys::asm::util::mul288x288;

/// Given 288-bit value `a` and `n`, the P256 order:
/// 1. if `a >= n`, calculate `a - n`
/// 2. if `a < n`, calculate `a` (no-op)
///
/// # Inputs
/// `r0` through `r8` shall contain `a`.
///
/// # Return
/// On return, `r0` through `r8` shall contain the result of the operation.
///
/// > **Note**: all other registers are clobbered.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".p256-cortex-m4")]
pub unsafe extern "C" fn P256_reduce_mod_n_once() {
    naked_asm!(
        "
            push {{lr}}
            // frame push {{lr}}

            adr r10, {P256_order}
            ldm r10, {{r10,r11,r12,lr}}
            subs r0, r10
            sbcs r1, r1, r11
            sbcs r2, r2, r12
            sbcs r3, r3, lr
            sbcs r4, r4, #0xffffffff
            sbcs r5, r5, #0xffffffff
            sbcs r6, r6, #0
            sbcs r7, r7, #0xffffffff
            sbcs r8, r8, #0

            sbc r9, r9, r9 // sets r9 to -1 if input < n, else 0
            and r10, r9
            and r11, r9
            and r12, r9
            and lr, r9

            adds r0, r10
            adcs r1, r1, r11
            adcs r2, r2, r12
            adcs r3, r3, lr
            adcs r4, r4, r9
            adcs r5, r5, r9
            adcs r6, r6, #0
            adcs r7, r7, r9
            adcs r8, r8, #0

            pop {{pc}}
        ",
        P256_order = sym super::P256_ORDER,
    )
}

/// Given a number `a`, do something. TODO: figure out what the heck this actually does.
///
/// # Inputs
/// `r0` shall contain a valid `*mut [u32; 8]`.
///
/// `r1` shall contain `a`, a valid `*const [u32; 8]`
///
/// The pointers in `r0` and `r1` may overlap.
///
/// # Return
/// On return, the dereference of the input value of `r0` shall contain the result of the computation.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_reduce_mod_n_32bytes(res: *mut [u32; 8], a: *const [u32; 8]) {
    naked_asm!(
        "
            push {{r0, r4-r11, lr}}
            //frame push {{r0, r4-r11, lr}}
            // frame address sp, 40
            ldm r1, {{r0-r7}}
            mov r8, #0
            bl {P256_reduce_mod_n_once}
            pop {{r8}}
            // frame address sp, 36
            stm r8, {{r0-r7}}
            pop {{r4-r11,pc}}
        ",
        P256_reduce_mod_n_once = sym P256_reduce_mod_n_once,
    )
}

/// Given a number `a`, do something. TODO: figure out what the heck this actually does.
///
/// # Inputs
/// `r0` shall contain a valid `*mut [u8; 8]`.
///
/// `r1` shall contain `a`, a valid `*const [u8; 9]`.
///
/// # Return
/// On return, the dereference of the input value of `r0` shall contain the result of the operation.
///
/// > **Note**: this function clobbers all registers.
#[unsafe(link_section = ".p256-cortex-m4")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_reduce_mod_n_64bytes() {
    naked_asm!(
        "
            push {{r0, r4-r11, lr}}
            // frame push {{r0, r4-r11, lr}}
            sub sp, sp, #108
            // frame address sp,148

            mov r10, r1

            add r0, sp, #36
            adds r1, r1, #28
            adr r2, 0f
            bl {mul288x288}

            mov r0, sp
            add r1, sp, #72
            adr r2, {P256_order}
            bl {mul288x288}

            ldm r10, {{r0-r8}}
            pop {{r9-r12}}
            // frame address sp,132
            subs r0, r0, r9
            sbcs r1, r1, r10
            sbcs r2, r2, r11
            sbcs r3, r3, r12
            pop {{r9-r12, lr}}
            // frame address sp,112
            sbcs r4, r4, r9
            sbcs r5, r5, r10
            sbcs r6, r6, r11
            sbcs r7, r7, r12
            sbcs r8, r8, lr

            bl {P256_reduce_mod_n_once}
            bl {P256_reduce_mod_n_once}
            add sp, sp, #72
            // frame address sp,40
            pop {{r9}}
            // frame address sp,36

            stm r9, {{r0-r7}}

            pop {{r4-r11,pc}}

            .align 2
        0: // P256_order_mu
            .word 0xeedf9bfe
            .word 0x012ffd85
            .word 0xdf1a6c21
            .word 0x43190552
            .word 0xffffffff
            .word 0xfffffffe
            .word 0xffffffff
            .word 0x0
            .word 0x1
        ",
        mul288x288 = sym mul288x288,
        P256_reduce_mod_n_once = sym P256_reduce_mod_n_once,
        P256_order = sym super::P256_ORDER,
    )
}
