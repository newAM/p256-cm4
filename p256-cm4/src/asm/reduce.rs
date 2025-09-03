use core::arch::naked_asm;

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
