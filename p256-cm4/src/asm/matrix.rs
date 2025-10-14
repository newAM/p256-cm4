use core::arch::naked_asm;

/// The elements of a matrix.
///
/// These values are two's-complement encoded and in the range `[-2^30, 2^31]`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[expect(unused)]
struct MatrixElement(u32);

/// For [`MatrixElement`]s `a`, and `b`, and [`XYInteger`]s `x` and `y`, compute `a * x + b * y mod N` (where `N` is the order of the `p256` curve).
///
/// # Inputs
/// `r0` shall contain `a`, an `u32`.
///
/// `r1` shall contain `b`, an `u32`.
///
/// `r2` shall point to `xy`, a valid [`*const [XYInteger; 2]`](`crate::XYInteger`).
///
/// `r3` shall contain a valid [`*mut XYInteger`](`crate::XYInteger`) in which the result will be stored.
///
/// # Return
/// On return, the dereference of the input value of `r3` will contain the result of the computation.
///
/// # Safety
/// The caller must guarantee that `out` and `xy` are valid for the duration of the function call,
/// and that `out` is valid for writes.
///
/// > **Note**: This function adheres to the ARM calling convention.
// TODO: replace `u32` with `MatrixElement`.
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub(in crate::sys) unsafe extern "C" fn P256_matrix_mul_mod_n(
    a: u32,
    b: u32,
    xy: *const [crate::sys::XYInteger; 2],
    out: *mut crate::sys::XYInteger,
) {
    naked_asm!(
        "
                .align 2
                push {{r4-r11, lr}}
                // frame push {{r4-r11, lr}}

                // this function calculates a * x + b * y mod N (where N is the order of the P-256 curve)

                // the range is [-2^30, 2^31], so if negative, the top 2 bits are both 1s
                // convert to absolute value and sign
                and r4, r0, r0, lsl #1
                asrs r4, r4, #31
                eors r0, r0, r4
                subs r0, r0, r4

                and r5, r1, r1, lsl #1
                asrs r5, r5, #31
                eors r1, r1, r5
                subs r1, r1, r5

                ldm r2!, {{r6}} // x sign
                ldr r7, [r2, #32] // y sign

                // compute the resulting sign, which will be negative if exactly one of x'sign and y's sign is negative
                eors r4, r4, r6 // combine x's sign and a's sign
                eors r5, r5, r7 // combine y's sign and b's sign
                eors r4, r4, r5 // mask for negating a * x before adding to b * y
                stm r3!, {{r5}}
                push {{r1, r2,r3 }}
                // frame address sp,48

                // load x, which is stored as an unsigned 256-bit integer and initially conditionally negated through r6
                // now conditionally negate it depending on the r4 mask
                ldm r2, {{r1-r3, r5-r9}}
                eors r1, r1, r4
                eors r2, r2, r4
                eors r3, r3, r4
                eors r5, r5, r4
                eors r6, r6, r4
                eors r7, r7, r4
                eor  r8, r8, r4
                eor  r9, r9, r4

                subs r1, r1, r4
                sbcs r2, r2, r4
                sbcs r3, r3, r4
                sbcs r5, r5, r4
                sbcs r6, r6, r4
                sbcs r7, r7, r4
                sbcs r8, r8, r4
                sbcs r9, r9, r4

                sbcs r4, r4, r4 // if the value is nonzero, r4 will now contain -1 and we will add N to make it positive

                lsrs lr, r4, #31
                mov r12,# 0
                ldrd r10, r11, 0f
                umaal r1, r12, lr, r10
                umaal r2, r12, lr,r11
                ldrd r10, r11, 0f + 8
                umaal r3, r12, lr, r10
                umaal r5, r12, lr, r11
                umaal r6, r12, lr, r4
                umaal r7, r12, lr, r4
                mov r10,#0
                umaal r8, r12, lr, r10
                umaal r9, r12, lr, r4

                // calculate a * x, the result fits in 287 bits
                umull r11, lr,  r10, r10
                umull r10, lr,  r0,  r1
                umull r1,  r12, r11, r11
                umaal r11, lr,  r0,  r2
                umaal r1,  lr,  r0,  r3
                umull r2,  r3,  r12, r12
                umaal r2,  lr,  r0,  r5
                umaal r3,  lr,  r0,  r6
                umull r4,  r5,  r12, r12
                umaal r4,  lr,  r0,  r7
                umaal r5,  lr,  r0,  r8
                umaal r12, lr,  r0,  r9

                // add b*y, the result will fit in 288 bits
                pop {{r0, r6}}
                // frame address sp,40
                adds r6, r6, #36
                ldm r6!, {{r8, r9}}
                movs r7, #0
                umaal r10, r7, r0, r8
                umaal r11, r7, r0, r9
                ldm r6!, {{r8, r9}}
                umaal r1, r7, r0, r8
                umaal r2, r7, r0, r9
                ldm r6!, {{r8, r9}}
                umaal r3, r7, r0, r8
                umaal r4, r7, r0, r9
                ldm r6!, {{r8, r9}}
                umaal r5,  r7, r0, r8
                umaal r12, r7, r0, r9
                add lr, lr, r7

                // reduce modulo N using montgomery redc algorithm
                ldr r0, =0xee00bc4f // montgomery multiplication factor N' (when R = 2^32), N*N' = -1 mod R
                mul r0, r10, r0   // m = ((T mod R)N') mod R
                movs r6, #0				// need 4-byte alignment on next instruction
                ldrd r8, r9, 0f
                umaal r10, r6, r0, r8 // t = (T + mN) / R
                umaal r11, r6, r0, r9
                subs r11, r11, r8 // conditionally subtract by N unless we later find out the result becomes negative
                ldrd r8,r10, 0f + 8
                umaal r1, r6, r0, r8
                sbcs r1, r1, r9
                umaal r2, r6, r0, r10
                mov r9, #-1
                umaal r3, r6, r0, r9
                umaal r4, r6, r0, r9
                movs r7, #0
                umaal r5, r6, r0, r7
                umaal r12, r6, r0, r9
                umaal lr, r6, r7, r7
                sbcs r2, r2, r8
                sbcs r3, r3, r10
                sbcs r4, r4, r9
                sbcs r5, r5, r9
                sbcs r12, r12, r7
                sbcs lr, lr, r9
                sbcs r6, r6, r7 // if the result becomes negative, r6 becomes -1

                // conditionally add back N
                ldrd r0, r9, 0f
                lsrs r6, r6, #31
                umaal r7, r11, r6, r0
                umaal r1, r11, r6, r9
                umaal r2, r11, r6, r8
                umaal r3, r11, r6, r10
                rsbs r0, r6, #0
                umaal r4, r11,r6, r0
                umaal r5, r11, r6, r0
                mov r8, #0
                umaal r11, r12, r6, r8
                umaal r12, lr, r6, r0

                pop {{r6}}
                // frame address sp,36
                stm r6!, {{r7}}
                stm r6!, {{r1-r5, r11, r12}}

                pop {{r4-r11, pc}}

            // TODO: replace this with a static and forward it. Has issues: https://github.com/rust-lang/rust/issues/146061
            // with a `sym`.
                .align 2
            0: // P256_ord
                .word 0xFC632551
                .word 0xF3B9CAC2
                .word 0xA7179E84
                .word 0xBCE6FAAD
                .word 0xFFFFFFFF
                .word 0xFFFFFFFF
                .word 0
                .word 0xFFFFFFFF
                .word 0

                .ltorg
            ",
    )
}

/// Given values `delta`, `f` and `g`, perform some matrix computation.
///
/// TODO: figure out what this computes
///
/// # Inputs
/// Register `r0` shall contain `delta`, a 32-bit signed integer.
///
/// Register `r1` shall contain `f`, a 32-bit unsigned integer.
///
/// Register `r2` shall contain `g`, a 32-bit unsigned integer.
///
/// Register `r3` shall contain a valid `*mut [u32; 4]`
///
/// # Return
/// On return, `r0` will contain `delta`, and the dereference of the input value of `r3` shall contain the result
/// of the computation.
///
/// # Safety
/// The caller must guarantee that `res`, is valid for the duration
/// of the function call, and that `res` is valid for writes.
///
/// > **Note**: This function adheres to the ARM calling convention.
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub(in crate::sys) unsafe extern "C" fn P256_divsteps2_31(
    delta: i32,
    f: u32,
    g: u32,
    res: *mut [u32; 4],
) -> i32 {
    naked_asm!(
        "
            push {r3, r4-r8, lr}
            // frame push {r3, r4-r8, lr}
            // frame address sp,28

            // u,v,q,r
            movs r4, #1
            movs r5, #0
            movs r6, #0
            movs r7, #1

            // counter
            mov lr, #31

            0:
            subs r3, r0, #1
            lsl r12, r2, #31
            bic r3, r12, r3
            asrs r3, r3, #31 // mask
            lsr r8, r3, #31  // b

            // conditionally negate delta
            eors r0 ,r0 ,r3
            subs r0 ,r0 ,r3

            mul r12, r1, r3 // t = f * -b (= f * m)
            bics r1, r1, r3 // f &= ~m
            umlal r1, r12, r2, r8 // f += g * b
            umaal r2, r12, r2, r3 // g += t + g * -b (= g * m)

            mul r12, r4, r3
            bics r4, r4, r3
            umlal r4, r12, r6, r8
            umaal r6, r12, r6, r3

            mul r12, r5, r3
            bics r5, r5, r3
            umlal r5, r12, r7, r8
            umaal r7, r12, r7, r3

            ands r12, r2, #1 // g0 = g & 1
            adds r0, r0, #1 // delta += 1

            // g = (g + g0 * f) / 2
            mul r3, r12, r1
            adds r2, r2, r3
            lsrs r2, r2, #1 // we don't need the MSB

            umlal r6, r8, r12, r4 // q += g0 * u
            umlal r7, r8, r12, r5 // r += g0 * v

            adds r4, r4, r4 // u *= 2
            adds r5, r5, r5 // v *= 2

            subs lr, lr, #1
            bne 0b

            pop {r3}
            stm r3!, {r4-r7}

            pop {r4-r8,pc}
        ",
        options(raw)
    )
}

/// Given inputs `a`, `b`, `f` and `g`, calculate `a * f + b * g`
///
/// # Inputs
/// `r0` shall contain `a`, a 32 bit unsigned integer.
///
/// `r1` shall contain `b`, a 32 bit unsigned integer.
///
/// `r2` shall contain `f` and `g`, two 257 bit signed numbers, as a [`*const [FGInteger; 2]`](crate::FGInteger).
///
/// `r3` shall contain a valid [`*mut FGInteger`](crate::FGInteger).
///
/// # Return
/// On return, the dereference of the input value of `r3` will contain the result of the operation.
///
/// # Safety
/// The caller must guarantee that `fg` and `res` are valid for the duration of the function call,
/// and that `res` is valid for writes.
///
/// > **Note**: This function adheres to the ARM calling convention.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub(in crate::sys) unsafe extern "C" fn P256_matrix_mul_fg_9(
    a: u32,
    b: u32,
    fg: *const [crate::sys::FGInteger; 2],
    res: *mut crate::sys::FGInteger,
) {
    naked_asm!(
        "
            push {r4-r11, lr}
            // frame push {r4-r11, lr}

            // this function calculates (a * f + b * g) / 2^31, which shall be an integer

            // the range is [-2^30, 2^31], so if negative, the top 2 bits are both 1s
            // convert to absolute value and sign
            and r4,  r0, r0, lsl #1
            asrs r4, r4, #31
            eors r0, r0, r4
            subs r0, r0, r4

            and r5,  r1, r1, lsl #1
            asrs r5, r5, #31
            eors r1, r1, r5
            subs r1, r1, r5

            ldm r2!, {r6} // f sign
            ldr r7, [r2, #36] // g sign

            // compute the resulting sign, which will be negative if exactly one of g'sign and b's sign is negative
            eors r4, r4, r6 // combine f's sign and a's sign
            eors r5, r5, r7 // combine g's sign and b's sign
            eors r4, r4, r5 // mask for negating a * f before adding to b * g
            stm r3!, {r5}
            push {r1, r2, r3}
            // frame address sp,48

            // load f, which is stored as a signed 257-bit number (sign extended to 288 bits) and initially conditionally negated through r6
            // now conditionally negate it depending on the r4 mask
            ldm r2!, {r1, r3, r5-r11}
            eors r1,  r1,  r4
            eors r3,  r3,  r4
            eors r5,  r5,  r4
            eors r6,  r6,  r4
            eors r7,  r7,  r4
            eor  r8,  r8,  r4
            eor  r9,  r9,  r4
            eor  r10, r10, r4

            subs r1,  r1,  r4
            sbcs r3,  r3,  r4
            sbcs r5,  r5,  r4
            sbcs r6,  r6,  r4
            sbcs r7,  r7,  r4
            sbcs r8,  r8,  r4
            sbcs r9,  r9,  r4
            sbcs r10, r10, r4
            // f is never 0, so we can skip last sbcs (for r11), since we know carry flag would be 0
            eor r4, r4, r11

            // multiply the signed 257-bit value by |a| (|a| <= 2^31), to get a signed 288-bit result
            umull r1, lr, r0, r1
            movs r2, #0
            umull r11, r12, r2,  r2
            umaal r2,  lr,  r0,  r3
            umaal r11, lr,  r0,  r5
            umull r3,  r5,  r12, r12
            umaal r3,  lr,  r0,  r6
            umaal r5,  lr,  r0,  r7
            umull r6,  r7,  r12, r12
            umaal r6,  lr,  r0,  r8
            umaal r7,  lr,  r0,  r9
            umaal r12, lr,  r0,  r10
            mla lr, r0, r4, lr
            // result: r1, r2, r11, r3, r5, r6, r7, r12, lr

            // add b*g (which also fits in a signed 288-bit value) and divide by 2^31 (the low 31 bits will all be zero before div)
            pop {r0, r4}
            // frame address sp,40
            adds r4, r4, #40
            ldm r4!, {r8, r9}
            mov r10, #0
            umaal r1, r10, r0, r8
            umaal r2, r10, r0, r9
            adds r1, r1, r1
            adcs r2, r2, r2
            ldm r4!, {r1, r8, r9}
            umaal r10, r11, r0, r1
            umaal r11, r3, r0, r8
            umaal r3, r5, r0, r9
            adcs r10, r10, r10
            adcs r11, r11, r11
            adcs r3, r3 ,r3
            ldm r4, {r1, r4, r8, r9}
            umaal r5,  r6,  r0, r1
            umaal r6,  r7,  r0, r4
            umaal r7,  r12, r0, r8
            umaal r12, lr,  r0, r9 // by divsteps2 invariant, lr will now be 0 since both f and g each fits in a signed 257-bit value
            adcs r5, r5, r5
            adcs r6, r6, r6
            adcs r7, r7, r7
            adcs r12, r12, r12
            sbcs lr, lr, lr // extract the sign bit and sign-extend it
            mvn lr, lr
            pop {r1}
            //frame address sp,36
            stm r1!, {r2, r10, r11}
            stm r1!, {r3, r5, r6, r7, r12, lr}

            pop {r4-r11, pc}
        ",
        options(raw),
    );
}
