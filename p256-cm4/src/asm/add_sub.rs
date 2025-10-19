use core::arch::naked_asm;

/// Compute `A - B mod p`, assuming that `A, B < p`, and `p` is the `p256` prime.
///
/// # Inputs
/// `r1` shall contain `A`, a valid [`*const Montgomery`](`super::Montgomery`).
///
/// `r2` shall contain `B`, a valid [`*const Montgomery`](`super::Montgomery`).
///
/// # Return
/// On return, `r0` through `r7` will contain the resulting [`Montgomery`](super::Montgomery).
///
/// All other registers are clobbered.
///
/// # Safety
/// The caller must ensure that the ABI for this function is upheld. It is impossible to do so from
/// normal rust code, so it must only be called from other inline assembly.
// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_submod() {
    naked_asm!(
        "
            ldm r1, {{r3-r10}}
            ldm r2!, {{r0,r1,r11,r12}}
            subs r3, r0
            sbcs r4, r4, r1
            sbcs r5, r5, r11
            sbcs r6, r6, r12
            ldm r2, {{r0, r1, r11, r12}}
            sbcs r7, r7, r0
            sbcs r8, r8, r1
            sbcs r9, r9, r11
            sbcs r10, r10, r12

            sbcs r11,r11,r11

            adds r0, r3, r11
            adcs r1, r4, r11
            adcs r2, r5, r11
            adcs r3, r6, #0
            adcs r4, r7, #0
            adcs r5, r8, #0
            adcs r6, r9, r11, lsr #31
            adcs r7, r10, r11

            bx lr
        "
    )
}

/// Computes `A + B mod p`, assuming that `A, B < p`, and `p` is the `p256` prime.
///
/// # Inputs
/// `r1` shall contain `A`, a valid [`*const Montgomery`](`super::Montgomery`).
///
/// `r2` shall contain `B`, a valid [`*const Montgomery`](`super::Montgomery`).
///
/// # Return
/// On return, `r0` through `r7` will contain the resulting [`Montgomery`](super::Montgomery).
///
/// All other registers are clobbered.
///
/// # Safety
/// The caller must ensure that the ABI for this function is upheld. It is impossible to do so from
/// normal rust code, so it must only be called from other inline assembly.
// TODO: make this `extern "custom"` once that is stabilized (https://github.com/rust-lang/rust/issues/140829)
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_addmod() {
    naked_asm!(
        "
            ldm r2, {{r2-r9}}
            ldm r1!, {{r0, r10, r11, r12}}
            adds r2, r0
            adcs r3, r3, r10
            adcs r4, r4, r11
            adcs r5, r5, r12
            ldm r1, {{r0, r1, r11, r12}}
            adcs r6,  r6,  r0
            adcs r7,  r7,  r1
            adcs r8,  r8,  r11
            adcs r9,  r9,  r12
            movs r10, #0
            adcs r10, r10, r10

            subs r2,  #0xffffffff
            sbcs r3,  r3,  #0xffffffff
            sbcs r4,  r4,  #0xffffffff
            sbcs r5,  r5,  #0
            sbcs r6,  r6,  #0
            sbcs r7,  r7,  #0
            sbcs r8,  r8,  #1
            sbcs r9,  r9,  #0xffffffff
            sbcs r10, r10, #0

            adds r0, r2, r10
            adcs r1, r3, r10
            adcs r2, r4, r10
            adcs r3, r5, #0
            adcs r4, r6, #0
            adcs r5, r7, #0
            adcs r6, r8, r10, lsr #31
            adcs r7, r9, r10

            bx lr
        "
    )
}

/// Given two numbers `a` and `b`, compute `a + b mod n` where `n` is the P256 order.
///
/// # Inputs
/// `r0` shall contain a valid `*mut [u32; 8]`.
///
/// `r1` shall contain `a`, a valid `*const [u32; 8]`.
///
/// `r2` shall contain `b`, a valid `*const [u32; 8]`.
///
/// # Return
/// On return, the dereference of the input value of `r0` will contain the result of the computation.
///
/// > **Note**: `r0` will be overriden during the execution of this function (it is callee-saved).
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub(in crate::sys) unsafe extern "C" fn P256_add_mod_n(
    res: *mut [u32; 8],
    a: *const [u32; 8],
    b: *const [u32; 8],
) {
    naked_asm!(
        "
            push {{r0, r4-r11, lr}}
            // frame push {{r0, r4-r11,lr}}
            // frame address sp,40

            mov r12, r1

            ldm r2, {{r4-r11}}
            ldm r12!, {{r0-r3}}
            adds r0, r4
            adcs r1, r1, r5
            adcs r2, r2, r6
            adcs r3, r3, r7
            ldm r12, {{r4-r7}}
            adcs r4, r4, r8
            adcs r5, r5, r9
            adcs r6, r6, r10
            adcs r7, r7, r11
            movs r8, #0
            adcs r8, r8, r8

            bl {P256_reduce_mod_n_once}
            bl {P256_reduce_mod_n_once}
            pop {{r8}}
            // frame address sp,36
            stm r8, {{r0-r7}}

            pop {{r4-r11, pc}}
        ",
        P256_reduce_mod_n_once = sym super::P256_reduce_mod_n_once,
    )
}
