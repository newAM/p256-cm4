use core::arch::naked_asm;

use crate::sys::asm::Montgomery;

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

/// For input `a`, calculate `a * 2 mod p`, where `p` is the P256 prime.
///
/// # Inputs
/// `r0` through `r7` shall contain `a`, a 256 bit number in little-endian order.
///
/// # Outputs
/// `r0` through `r7` will contain the result of the computation.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_times2() {
    naked_asm!(
        "
        	adds r0,r0
            adcs r1,r1,r1
            adcs r2,r2,r2
            adcs r3,r3,r3
            adcs r4,r4,r4
            adcs r5,r5,r5
            adcs r6,r6,r6
            adcs r7,r7,r7
            mov r8,#0
            adcs r8,r8,r8

            subs r0,#0xffffffff
            sbcs r1,r1,#0xffffffff
            sbcs r2,r2,#0xffffffff
            sbcs r3,r3,#0
            sbcs r4,r4,#0
            sbcs r5,r5,#0
            sbcs r6,r6,#1
            sbcs r7,r7,#0xffffffff
            sbcs r8,r8,#0

            adds r0,r8
            adcs r1,r1,r8
            adcs r2,r2,r8
            adcs r3,r3,#0
            adcs r4,r4,#0
            adcs r5,r5,#0
            adcs r6,r6,r8, lsr #31
            adcs r7,r7,r8

            bx lr
        "
    )
}

/// Given inputs `a` (a point in Jacobian form with integers in montgomery form) and `b` (a point in Jacobian form with integers in montgomery form, or an affine point):
/// 1. if `is_sub == false`, calculate `a + b`
/// 2. If `is_sub == true`, calculate `a - b`
///
/// # Inputs
/// `r0` shall contain `a`, a valid [`*mut [Montgomery; 3]`](super::Montgomery). This point is treated as the point at infinity if its Z-coordinate is 0.
///
/// `r1` shall contain `b`, which must not be the point at infinity:
/// 1. if `r3 == 0`, a valid [`*const [Montgomery; 3]`](super::Montgomery).
/// 2. else, a valid `*const [[u32; 8]; 2]`.
///
/// `r2` shall contain `is_sub`, a boolean.
///
/// `r3` shall contain a boolean, set to `true` if `b` is affine, and `false` if it is in jacobian form.
///
/// The pointers in `r0` and `r1` _**must not**_ overlap.
///
/// # Return
/// On return, the location pointed to by the input `r0` will contain the result of the operation.
///
/// > **Note**: `r0` will be overriden during the execution of this function (it is callee-saved).
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_add_sub_j(
    a: *mut [Montgomery; 3],
    b: *const u32,
    is_sub: bool,
    b_is_affine: bool,
) {
    naked_asm!(
        "
            push {{r0-r11, lr}}
            // frame push {{r0-r11, lr}}
            // frame address sp, 52

            // ldr r4, [r0, #64]
            // cbnz r4, 2f
            add r4, r0, #64
            ldm r4, {{r4-r11}}
            orrs r4, r5
            orrs r4, r6
            orrs r4, r7
            orrs r4, r8
            orrs r4, r9
            orrs r4, r10
            orrs r4, r11
            bne 2f

            // First point is 0, so just set result to (-) the other point
            bl {add_sub_helper}
            add sp, #16
            // frame address sp,36
            pop {{r4-r11, pc}}
        2:
            // frame address sp,52
            // Here a variant of
            // https://www.hyperelliptic.org/EFD/g1p/auto-code/shortw/jacobian-3/addition/add-1998-cmo-2.op3
            // is used, but rearranged and uses less temporaries.
            // The first operand to the function is both (X3,Y3,Z3) and (X2,Y2,Z2).
            // The second operand to the function is (X1,Y1,Z1)

            cbnz r3,100f

            // Z1Z1 = Z1^2
            adds r1, #64
            ldm r1, {{r0-r7}}
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,84

            // U2 = X2*Z1Z1
            ldr r1, [sp, #32]
            mov r2, sp
            bl {P256_mulmod}
            ldr r8, [sp, #32]
            stm r8, {{r0-r7}}

            // t1 = Z1*Z1Z1
            ldr r1, [sp, #36]
            adds r1, #64
            mov r2, sp
            bl {P256_mulmod}
            stm sp, {{r0-r7}}

            // S2 = Y2*t1
            ldr r1, [sp, #32]
            adds r1, #32
            mov r2, sp
            bl {P256_mulmod}
            ldr r8, [sp, #32]
            add r8, #32
            stm r8, {{r0-r7}}
            b 101f
        100:
            sub sp, #32
            // frame address sp,84
        101:

            // Z2Z2 = Z2^2
            ldr r1, [sp, #32]
            adds r1, #64
            ldm r1, {{r0-r7}}
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,116

            // U1 = X1*Z2Z2
            ldr r1, [sp, #68]
            mov r2, sp
            bl {P256_mulmod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            // t2 = Z2*Z2Z2
            ldr r1, [sp, #64]
            adds r1, #64
            mov r2, sp
            bl {P256_mulmod}
            stm sp, {{r0-r7}}

            // S1 = Y1*t2
            ldr r1, [sp, #68]
            adds r1, #32
            mov r2, sp
            bl {P256_mulmod}
            stm sp,{{r0-r7}}

            // H = U2-U1
            ldr r1, [sp, #64]
            add r2, sp, #32
            bl {P256_submod}
            ldr r8, [sp,#64]
            stm r8, {{r0-r7}}

            // HH = H^2
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,148

            // Z3 = Z2*H
            ldr r2, [sp, #96]
            add r1, r2, #64
            bl {P256_mulmod}
            ldr r8,[ sp, #96]
            add r8, #64
            stm r8, {{r0-r7}}

            // Z3 = Z1*Z3
            ldr r1, [sp, #108]
            cbnz r1, 102f
            ldr r1, [sp, #100]
            adds r1, #64
            mov r2, r8
            bl {P256_mulmod}
            ldr r8, [sp, #96]
            add r8, #64
            stm r8, {{r0-r7}}
        102:

            // HHH = H*HH
            ldr r1, [sp, #96]
            mov r2, sp
            bl {P256_mulmod}
            ldr r8, [sp,#96]
            stm r8, {{r0-r7}}

            //cbnz r0,3f
            orrs r1, r0
            orrs r1, r2
            orrs r1, r3
            orrs r1, r4
            orrs r1, r5
            orrs r1, r6
            orrs r0, r1, r7
        3:
            push {{r0}} // if r0 == 0: HHH is 0, which means the two input points have the same affine x coordinates
            // frame address sp, 152

            // r = S2-+S1
            ldr r1, [sp, #100]
            adds r1, #32
            add r2, sp, #36
            ldr r3, [sp, #108]
            cbz r3, 4f
            bl {P256_addmod}
            b 5f
        4:
            bl {P256_submod}
        5:
            ldr r8, [sp, #100]
            add r8, #32
            stm r8, {{r0-r7}}

            // check r == 0 && HHH == 0
            pop {{r8}}
            // frame address sp,148
            // cbnz r0,6f
            orrs r1, r0
            orrs r1, r2
            orrs r1, r3
            orrs r1, r4
            orrs r1, r5
            orrs r1, r6
            orrs r1, r7
            orrs r1, r8
            bne 6f
            // Points should be doubled since addition formula can't handle this case
            // Since we have already overwritten the first point,
            // we must copy the second point after possibly negating it
            add sp,#96
            // frame address sp,52
            ldm sp, {{r0-r3}}
            bl {add_sub_helper}

            ldr r0, [sp, #0]
            mov r1, r0
            add sp, #16
            // frame address sp,36
            bl {P256_double_j}
            pop {{r4-r11, pc}}
        6:
            // frame address sp,148

            // V = U1*HH
            add r1, sp, #64
            mov r2, sp
            bl {P256_mulmod}
            add r8, sp, #64
            stm r8, {{r0-r7}}

            // t3 = r^2
            ldr r0, [sp, #96]
            adds r0, #32
            ldm r0, {{r0-r7}}
            bl {P256_sqrmod}
            stm sp, {{r0-r7}}

            // t2 = S1*HHH
            add r1, sp, #32
            ldr r2, [sp, #96]
            bl {P256_mulmod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            // X3 = t3-HHH
            mov r1, sp
            ldr r2, [sp, #96]
            bl {P256_submod}
            ldr r8, [sp, #96]
            stm r8, {{r0-r7}}

            // t3 = 2*V
            add r0, sp, #64
            ldm r0, {{r0-r7}}
            bl {P256_times2}
            stm sp, {{r0-r7}}

            // X3 = X3-t3
            ldr r1, [sp, #96]
            mov r2, sp
            bl {P256_submod}
            ldr r8, [sp, #96]
            stm r8, {{r0-r7}}

            // t3 = V-X3
            add r1, sp, #64
            ldr r2, [sp, #96]
            bl {P256_submod}
            stm sp, {{r0-r7}}

            // t3 = r*t3
            ldr r1, [sp, #96]
            adds r1, #32
            mov r2, sp
            bl {P256_mulmod}
            stm sp, {{r0-r7}}

            // Y3 = t3-+t2
            ldr r0, [sp, #104]
            mov r1, sp
            add r2, sp, #32
            cbz r0, 7f
            bl {P256_addmod}
            b 8f
        7:
            bl {P256_submod}
        8:
            ldr r8, [sp, #96]
            add r8, #32
            stm r8, {{r0-r7}}

            add sp, #112
            // frame address sp,36

            pop {{r4-r11, pc}}
        ",
        P256_mulmod = sym super::P256_mulmod,
        P256_sqrmod = sym super::P256_sqrmod,
        P256_addmod = sym super::P256_addmod,
        P256_submod = sym super::P256_submod,
        P256_times2 = sym P256_times2,
        add_sub_helper = sym super::add_sub_helper,
        P256_double_j = sym P256_double_j,
    )
}

/// Given jacobian (with integers in montgomery form) `a`, calculate the affine `x` and `y` points.
///
/// # Inputs
/// `r0` shall contain `x`, a valid [`*mut Montomery`](super::Montgomery).
///
/// `r1` shall contain `y`, a valid [`*mut Montomery`](super::Montgomery).
///
/// `r2` shall contain `a`, a valid [`*const [Montgomery; 3]`](super::Montgomery).
///
/// # Return
/// On return, the locations pointed to by the input `r0` and `r0` will contain the results of the operation.
///
/// > **Note**: `r0` will be overriden during the execution of this function (it is callee-saved).
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_jacobian_to_affine(
    x: *mut Montgomery,
    y: *mut Montgomery,
    a: *const [Montgomery; 3],
) {
    naked_asm!(
        "
            push {{r0, r1, r2, r4-r11, lr}}
            // frame push {{r0, r1, r2, r4-r11,lr}}
            // frame address sp,48

            adds r2, #64
            ldm r2, {{r0-r7}}
            mov r8, #0
            bl {P256_modinv_sqrt}
            push {{r0-r7}}
            // frame address sp,80

            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,112

            add r1, sp,# 32
            mov r2, sp
            bl {P256_mulmod}
            add r8, sp, #32
            stm r8, {{r0-r7}}

            mov r1,sp
            ldr r2, [sp, #72]
            bl {P256_mulmod}
            ldr r8, [sp,#64]
            stm r8, {{r0-r7}}

            ldr r2, [sp,#72]
            add r1, sp, #32
            adds r2, r2, #32
            bl {P256_mulmod}
            ldr r8, [sp,#68]
            stm r8, {{r0-r7}}

            add sp, #76
            // frame address sp,36

            pop {{r4-r11, pc}}
        ",
        P256_modinv_sqrt = sym super::P256_modinv_sqrt,
        P256_sqrmod = sym super::P256_sqrmod,
        P256_mulmod = sym super::P256_mulmod,
    )
}
