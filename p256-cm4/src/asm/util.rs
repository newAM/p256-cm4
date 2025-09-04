use core::arch::naked_asm;

/// Given two 288 bit numbers `a` and `b`, calculate `a * b`.
///
/// # Inputs
/// `r0` shall contain a valid `*mut [u32; 9]`.
///
/// `r1` shall contain `a`, a valid `*const [u32; 9]`.
///
/// `r2` shall contain `b`, a valid `*const [u32; 9]`.
///
/// # Return
/// On return, the location pointed to by `r0` will contain the result of the computation.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mul288x288() {
    naked_asm!(
        "
            push {r4-r11,lr}
            // frame push {r4-r11,lr}

            mov r4,r0
            mov r5,r2
            mov r6,r1

            movs r1,#72
            bl setzero

            ldm r5,{r0-r2,r8-r12,lr}

            movs r7,#9
        0:
            ldm r6!,{r5}
            push {r6,r7}
            //frame address sp,44
            movs r3,#0
            ldm r4,{r6,r7}
            umaal r6,r3,r5,r0
            umaal r7,r3,r5,r1
            stm r4!,{r6,r7}
            ldm r4,{r6,r7}
            umaal r6,r3,r5,r2
            umaal r7,r3,r5,r8
            stm r4!,{r6,r7}
            ldm r4,{r6,r7}
            umaal r6,r3,r5,r9
            umaal r7,r3,r5,r10
            stm r4!,{r6,r7}
            ldm r4,{r6,r7}
            umaal r6,r3,r5,r11
            umaal r7,r3,r5,r12
            stm r4!,{r6,r7}
            ldm r4,{r6}
            umaal r3,r6,r5,lr
            stm r4!,{r3,r6}

            subs r4,r4,#36
            pop {r6,r7}
            //frame address sp,36
            subs r7,r7,#1
            bne 0b

            pop {r4-r11,pc}
        ",
        options(raw)
    )
}

/// Given jacobian points (with integers in montgomery form) `a`:
/// 1. If `negate_y == true`, negate `a`
/// 2. If `set_z_to_one == true`, set the Z coordinate of `a` to 1
///
/// # Inputs
/// `r0` shall contain a valid [`*mut [Montgomery; 3]`](super::Montgomery).
///
/// `r1` shall contain `a`, a valid [`*const [Montgomery; 3]`](super::Montgomery).
///
/// `r2` shall contain `negate_y`, a boolean.
///
/// `r3` shall contain `set_z_to_one`, a boolean.
///
/// # Return
/// On return, the dereference of the input value of `r0` shall contain the result of the operation.
///
/// > **Note**: this function clobbers all registers.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_sub_helper() {
    naked_asm!(
        "
        	push {lr}
            // frame push {lr}
            ldm r1!, {r5-r12}
            stm r0!, {r5-r12}
            ldm r1!, {r5-r12}
            cbz r2, 0f
            // note that Y is never 0 for a valid point
            mov lr, #0
            rsbs r4,  r2, #0
            subs r5,  r4, r5
            sbcs r6,  r4, r6
            sbcs r7,  r4, r7
            sbcs r8,  lr, r8
            sbcs r9,  lr, r9
            sbcs r10, lr, r10
            sbcs r11, r2, r11
            sbcs r12, r4, r12
        0:
            stm r0!, {r5-r12}
            cbnz r3, 1f
            ldm r1, {r5-r12}
            stm r0, {r5-r12}
            b 2f
        1:
            // Set Z3 to 1 in Montgomery form
            movs r4, #0
            umull r5, r10, r4, r4
            mvns r6, r4
            mvns r7, r4
            mov r8, #0xffffffff
            mov r9, #0xfffffffe

            stm r0,{r3-r10}
        2:
            pop {pc}
        ",
        options(raw)
    )
}

/// Given a number `a`, validate that it is in the range `[1, n-1]` where `n` is the P256 order.
///
/// # Inputs
/// `r0` shall contain `a`, a valid `*const [u32; 8]`.
///
/// # Return
/// On return, `r0` will contain `1` if `a` was in the range, and `0` otherwise.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".p256-cortex-m4")]
pub unsafe extern "C" fn P256_check_range_n(a: *const [u32; 8]) -> bool {
    naked_asm!(
        "
            push {{r4-r11, lr}}
            // frame push {{r4-r11, lr}}
            ldm r0, {{r1-r8}}
            orrs r0, r1, r2
            orrs r0, r3
            orrs r0, r4
            orrs r0, r5
            orrs r0, r6
            orrs r0, r7
            orrs r0, r8
            beq 0f

            adr r0, {P256_ORDER}
            ldm r0!, {{r9-r12}}
            subs r1, r9
            sbcs r2, r2, r10
            sbcs r3, r3, r11
            sbcs r4, r4, r12
            ldm r0,{{r0-r3}}
            sbcs r5, r5, r0
            sbcs r6, r6, r1
            sbcs r7, r7, r2
            sbcs r8, r8, r3

            sbcs r0, r0, r0
            lsrs r0,#31
        0:
            pop {{r4-r11, pc}}
        ",
        P256_ORDER = sym super::P256_ORDER
    )
}

/// Given input `n`, zero `n * 8` bytes.
///
/// # Inputs
/// `r0` shall contain a valid, word-aligned `*mut u8` that is valid for writes of `n * 8` bytes.
///
/// `r1` shall contain `n * 8` (this value _must_ be a multiple of 8), i.e. 16 to clear 16 bytes.
///
/// # Returns
/// This function does not return any values.
///
/// This function clobbers r0, r1, r2 and r3.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setzero() {
    naked_asm!(
        "
            movs r2, #0
            movs r3, #0
        0:
            stm r0!, {r2,r3}
            subs r1, r1, #8
            bne 0b
            bx lr
        ",
        options(raw)
    )
}
