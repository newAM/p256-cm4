use core::arch::naked_asm;

/// For input `a = A * R mod p` and mode `mode`:
/// 1. If `mode == modinv`, calculate `A^-1 * R mod p = (a/R)^-1 * R mod p = R^2 / a mod p`
/// 2. If `mode == sqrt`, calculate `sqrt(A) * R mod p`
///
/// # Inputs
/// `r0` through `r7` shall contain `a`, a [`Montgomery`](super::Montgomery).
///
/// `r8` shall contain `mode`, where `0 = modinv` and `1 = sqrt`.
///
/// # Outputs
/// `r0` through `r7` shall contain the result of the computation.
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_modinv_sqrt() {
    naked_asm!(
        "
            push {{r0-r8, lr}}

            // t = a^2*a
            mov r8, #1
            mov r9, sp
            bl {P256_sqrmod_many_and_mulmod}
            push {{r0-r7}}

            // a4_2 = a2_0^(2^2)
            bl {P256_sqrmod}
            bl {P256_sqrmod}
            push {{r0-r7}}

            // a4_0 = a4_2*a2_0
            mov r1,sp
            add r2,sp,#32
            bl {P256_mulmod}
            add r8,sp,#32
            stm r8,{{r0-r7}}

            // a8_0 = a4_0^(2^(8-4))*a4_0
            mov r8,#8-4
            add r9,sp,#32
            bl {P256_sqrmod_many_and_mulmod}
            push {{r0-r7}}

            // a16_0 = a8_0^(2^(16-8))*a8_0
            mov r8,#16-8
            mov r9,sp
            bl {P256_sqrmod_many_and_mulmod}
            push {{r0-r7}}

            // a32_0 = a16_0^(2^(32-16))*a16_0
            mov r8,#16
            mov r9,sp
            bl {P256_sqrmod_many_and_mulmod}
            push {{r0-r7}}

            // t = a32_0^(2^(64-32))*a
            mov r8,#32
            add r9,sp,#5*32
            bl {P256_sqrmod_many_and_mulmod}

            ldr r8,[sp,#6*32]
            cmp r8,#0
            bne 0f

            // t = t^(2^(192-64))*a32_0
            mov r8,#192-64
            mov r9,sp
            bl {P256_sqrmod_many_and_mulmod}

            // t = t^(2^(224-192))*a32_0
            mov r8,#224-192
            mov r9,sp
            bl {P256_sqrmod_many_and_mulmod}

            // t = t^(2^(240-224))*a16_0
            mov r8,#240-224
            add r9,sp,#32
            bl {P256_sqrmod_many_and_mulmod}

            // t = t^(2^(248-240))*a8_0
            mov r8,#248-240
            add r9,sp,#64
            bl {P256_sqrmod_many_and_mulmod}

            // t = t^(2^(252-248))*a4_0
            mov r8,#252-248
            add r9,sp,#128
            bl {P256_sqrmod_many_and_mulmod}

            // t = t^(2^(256-252))*a4_2
            mov r8,#256-252
            add r9,sp,#96
            bl {P256_sqrmod_many_and_mulmod}
            stm sp,{{r0-r7}}

            // r = t*a
            mov r1,sp
            add r2,sp,#5*32
            bl {P256_mulmod}
            b 1f

        0:
            // t = t^(2^(160-64))*a
            mov r8,#160-64
            add r9,sp,#5*32
            bl {P256_sqrmod_many_and_mulmod}

            // t = t^(2^(254-160))
            mov r8,#254-160
            bl {P256_sqrmod_many}
        1:

            add sp,#6*32+4

            pop {{pc}}
        ",
        P256_sqrmod_many_and_mulmod = sym super::P256_sqrmod_many_and_mulmod,
        P256_mulmod = sym super::P256_mulmod,
        P256_sqrmod_many = sym super::P256_sqrmod_many,
        P256_sqrmod = sym super::P256_sqrmod,
    )
}
