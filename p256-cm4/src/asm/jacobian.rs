use core::arch::naked_asm;

use crate::asm::Montgomery;

/// Doubles a point in Jacobian form (with integers Montgomery form).
///
/// # Inputs
/// `r0` shall contain a valid [`*mut [Montgomery; 3]`](super::Montgomery).
///
/// `r1` shall contain a valid [`*const [Montgomery; 3`](super::Montgomery), containing the point to be doubled.
///
/// `r0` and `r1` may overlap.
///
/// # Return
/// On return, the dereference of the input value of `r0` will contain the result of the computation.
///
/// > **Note**: `r0` will be overriden during the execution of this function (it is callee-saved).
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_double_j(
    jacobian_out: *mut [Montgomery; 3],
    jacobian_in: *const [Montgomery; 3],
) {
    naked_asm!(
        "
            push {{r0, r1, r4-r11, lr}}
            // frame push {{r0, r1, r4-r11, lr}}
            // frame address sp,44

            // https://eprint.iacr.org/2014/130.pdf, algorithm 10

            // t1 = Z1^2
            adds r1, #64
            ldm r1, {{r0-r7}}
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,76

            // Z2 = Y1 * Z1
            ldr r1, [sp, #36]
            adds r1, #32
            add r2, r1, #32
            bl {P256_mulmod}
            ldr r8, [sp, #32]
            add r8, #64
            stm r8, {{r0-r7}}

            // t2 = X1 + t1
            ldr r1, [sp, #36]
            mov r2, sp
            bl {P256_addmod}
            push {{r0-r7}}
            // frame address sp,108

            // t1 = X1 - t1
            ldr r1, [sp, #68]
            add r2, sp, #32
            bl {P256_submod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            // t1 = t1 * t2
            add r1, sp, #32
            mov r2, sp
            bl {P256_mulmod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            // t2 = t1 / 2
            lsl r8,r0,#31
            adds r0, r0, r8, asr #31
            adcs r1, r1, r8, asr #31
            adcs r2, r2, r8, asr #31
            adcs r3, r3, #0
            adcs r4, r4, #0
            adcs r5, r5, #0
            adcs r6, r6, r8, lsr #31
            adcs r7, r7, r8, asr #31
            rrxs r7, r7
            rrxs r6, r6
            rrxs r5, r5
            rrxs r4, r4
            rrxs r3, r3
            rrxs r2, r2
            rrxs r1, r1
            rrx r0, r0
            stm sp, {{r0-r7}}

            // t1 = t1 + t2
            add r1, sp, #32
            mov r2, sp
            bl {P256_addmod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            // t2 = t1^2
            bl {P256_sqrmod}
            stm sp, {{r0-r7}}

            // Y2 = Y1^2
            ldr r0, [sp, #68]
            adds r0, #32
            ldm r0, {{r0-r7}}
            bl {P256_sqrmod}
            ldr r8, [sp, #64]
            add r8, #32
            stm r8, {{r0-r7}}

            // t3 = Y2^2
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,140

            // Y2 = X1 * Y2
            ldrd r0, r1, [sp, #96]
            add r2, r0, #32
            bl {P256_mulmod}
            ldr r8, [sp, #96]
            add r8, #32
            stm r8, {{r0-r7}}

            // X2 = 2 * Y2
            bl {P256_times2}
            ldr r8, [sp, #96]
            stm r8, {{r0-r7}}

            // X2 = t2 - X2
            add r1, sp, #32
            mov r2, r8
            bl {P256_submod}
            ldr r8, [sp, #96]
            stm r8, {{r0-r7}}

            // t2 = Y2 - X2
            mov r2, r8
            add r1, r2, #32
            bl {P256_submod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            // t1 = t1 * t2
            add r1, sp, #64
            add r2, sp, #32
            bl {P256_mulmod}
            add r8, sp, #64
            stm r8, {{r0-r7}}

            // Y2 = t1 - t3
            add r1, sp, #64
            mov r2, sp
            bl {P256_submod}
            ldr r8,[sp, #96]
            add r8, #32
            stm r8, {{r0-r7}}

            add sp, #104
            //frame address sp,36

            pop {{r4-r11, pc}}
        ",
        P256_mulmod = sym super::P256_mulmod,
        P256_sqrmod = sym super::P256_sqrmod,
        P256_addmod = sym super::P256_addmod,
        P256_submod = sym super::P256_submod,
        P256_times2 = sym P256_times2,
    );
}

unsafe extern "C" {
    fn P256_times2();
}
