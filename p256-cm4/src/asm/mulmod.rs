use core::arch::naked_asm;

/// For inputs `A*R mod p` and `B*R mod p`, compute `A*B*R mod p`.
///
/// # Inputs
/// `r1` shall contain a valid `*const [u32; 8]`, whose dereference is `A*R mod p`. TODO: figure out ordering. Montgomery?
///
/// `r2` shall contain a valid `*const [u32; 8]`, whose dereference is `B*R mod p`. TODO: figure out ordering. Montgomery?
///
/// # Return
/// On return, `r0` through `r7` will contain `A*B*R mod p`.
///
/// All other registers are clobbered.
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_mulmod() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "fpu")] {
            naked_asm!(
                "
                    push {lr}
                    // frame push {lr}

                    vmov s4, r2
                    vldm r1, {s8-s15}

                    ldm r2, {r2, r3, r4, r5}

                    vmov r0, r10, s8, s9
                    umull r6, r1, r2, r0

                    umull r7, r12, r3, r0
                    umaal r7, r1,  r2, r10

                    vmov s0, s1, r6, r7

                    umull r8, r6, r4, r0
                    umaal r8, r1, r3, r10

                    umull r9, r7, r5, r0
                    umaal r9, r1, r4, r10

                    umaal r1, r7, r5, r10

                    vmov lr, r0, s10, s11

                    umaal r8,  r12, r2, lr
                    umaal r9,  r12, r3, lr
                    umaal r1,  r12, r4, lr
                    umaal r12, r7,  r5, lr

                    umaal r9,  r6, r2, r0
                    umaal r1,  r6, r3, r0
                    umaal r12, r6, r4, r0
                    umaal r6,  r7, r5, r0

                    vmov s2, s3, r8, r9

                    vmov r10, lr, s12, s13

                    mov r9, #0
                    umaal r1,  r9, r2, r10
                    umaal r12, r9, r3, r10
                    umaal r6,  r9, r4, r10
                    umaal r7,  r9, r5, r10

                    mov r10, #0
                    umaal r12, r10, r2, lr
                    umaal r6,  r10, r3, lr
                    umaal r7,  r10, r4, lr
                    umaal r9,  r10, r5, lr

                    vmov r8, s14
                    mov lr, #0
                    umaal lr,  r6, r2, r8
                    umaal r7,  r6, r3, r8
                    umaal r9,  r6, r4, r8
                    umaal r10, r6, r5, r8

                    //_ _ _ _ _ 6 10 9| 7 | lr 12 1 _ _ _ _

                    vmov r8, s15
                    mov r11, #0
                    umaal r7,  r11, r2, r8
                    umaal r9,  r11, r3, r8
                    umaal r10, r11, r4, r8
                    umaal r6,  r11, r5, r8

                    //_ _ _ _ 11 6 10 9| 7 | lr 12 1 _ _ _ _

                    vmov r2, s4
                    adds r2, r2, #16
                    ldm r2, {r2, r3, r4, r5}

                    vmov r8, s8
                    movs r0, #0
                    umaal r1, r0, r2, r8
                    vmov s4, r1
                    umaal r12, r0, r3, r8
                    umaal lr,  r0, r4, r8
                    umaal r0,  r7, r5, r8 // 7=carry for 9

                    //_ _ _ _ 11 6 10 9+7| 0 | lr 12 _ _ _ _ _

                    vmov r8, s9
                    movs r1, #0
                    umaal r12, r1, r2, r8
                    vmov s5, r12
                    umaal lr, r1, r3, r8
                    umaal r0, r1, r4, r8
                    umaal r1, r7, r5, r8 // 7=carry for 10

                    //_ _ _ _ 11 6 10+7 9+1| 0 | lr _ _ _ _ _ _

                    vmov r8, s10
                    mov r12, #0
                    umaal lr, r12, r2, r8
                    vmov s6, lr
                    umaal r0,  r12, r3, r8
                    umaal r1,  r12, r4, r8
                    umaal r10, r12, r5, r8 // 12=carry for 6

                    //_ _ _ _ 11 6+12 10+7 9+1| 0 | _ _ _ _ _ _ _

                    vmov r8, s11
                    mov lr, #0
                    umaal r0, lr, r2, r8
                    vmov s7, r0
                    umaal r1,  lr, r3, r8
                    umaal r10, lr, r4, r8
                    umaal r6,  lr, r5, r8 // lr=carry for saved

                    //_ _ _ _ 11+lr 6+12 10+7 9+1| _ | _ _ _ _ _ _ _

                    vmov r0, r8, s12, s13
                    umaal r1, r9, r2, r0
                    vmov s8, r1
                    umaal r9,  r10, r3, r0
                    umaal r10, r6,  r4, r0
                    umaal r11, r6,  r5, r0 // 6=carry for next

                    //_ _ _ 6 11+lr 10+12 9+7 _ | _ | _ _ _ _ _ _ _

                    umaal r9,  r7, r2, r8
                    umaal r10, r7, r3, r8
                    umaal r11, r7, r4, r8
                    umaal r6,  r7, r5, r8

                    vmov r0, r8, s14, s15
                    umaal r10, r12, r2, r0
                    umaal r11, r12, r3, r0
                    umaal r6,  r12, r4, r0
                    umaal r7,  r12, r5, r0

                    umaal r11, lr,  r2, r8
                    umaal lr,  r6,  r3, r8
                    umaal r6,  r7,  r4, r8
                    umaal r7,  r12, r5, r8

                    // 12 7 6 lr 11 10 9 s8 s7 s6 s5 s4 s3 s2 s1 s0

                    //now reduce
                    vmov s13, s14, r6, r7
                    vmov s15, r12

                    vmov r0, r1, s0, s1
                    vmov r2, r3, s2, s3
                    vmov r4, r5, s4, s5
                    vmov r6, r7, s6, s7
                    vmov r8, s8

                    mov r12, #0

                    adds r3, r0
                    adcs r4,  r4,  r1
                    adcs r5,  r5,  r2
                    adcs r6,  r6,  r0
                    adcs r7,  r7,  r1
                    adcs r8,  r8,  r0
                    adcs r9,  r9,  r1
                    adcs r10, r10, #0
                    adcs r11, r11, #0
                    adcs r12, r12, #0

                    adds r6, r3
                    adcs r7, r7, r4 // r4 instead of 0
                    adcs r8,  r8,  r2
                    adcs r9,  r9,  r3
                    adcs r10, r10, r2
                    adcs r11, r11, r3
                    adcs r12, r12, #0

                    subs r7,  r0
                    sbcs r8,  r8,  r1
                    sbcs r9,  r9,  r2
                    sbcs r10, r10, r3
                    sbcs r11, r11, #0
                    sbcs r12, r12, #0 // r12 is between 0 and 2

                    vmov r1, r2, s13, s14
                    vmov r3, s15

                    adds r0, lr, r12
                    adcs r1, r1, #0
                    mov r12, #0
                    adcs r12, r12,# 0

                    //adds r7,r4 (added above instead)
                    adcs r8,  r8,  r5
                    adcs r9,  r9,  r6
                    adcs r10, r10, r4
                    adcs r11, r11, r5
                    adcs r0,  r0,  r4
                    adcs r1,  r1,  r5
                    adcs r2,  r2,  r12
                    adcs r3,  r3,  #0
                    mov r12, #0
                    adcs r12, r12 ,#0

                    adcs r10, r10, r7
                    adcs r11, r11, #0
                    adcs r0,  r0,  r6
                    adcs r1,  r1,  r7
                    adcs r2,  r2,  r6
                    adcs r3,  r3,  r7
                    adcs r12, r12, #0

                    subs r11 ,r4
                    sbcs r0,  r0,  r5
                    sbcs r1,  r1,  r6
                    sbcs r2,  r2,  r7
                    sbcs r3,  r3,  #0
                    sbcs r12, r12, #0

                    // now (T + mN) / R is
                    // 8 9 10 11 0 1 2 3 12 (lsb -> msb)

                    subs r8,  r8,  #0xffffffff
                    sbcs r9,  r9,  #0xffffffff
                    sbcs r10, r10, #0xffffffff
                    sbcs r11, r11, #0
                    sbcs r4,  r0,  #0
                    sbcs r5,  r1,  #0
                    sbcs r6,  r2,  #1
                    sbcs r7,  r3,  #0xffffffff
                    sbc r12,  r12, #0

                    adds r0, r8,  r12
                    adcs r1, r9,  r12
                    adcs r2, r10, r12
                    adcs r3, r11, #0
                    adcs r4, r4,  #0
                    adcs r5, r5,  #0
                    adcs r6, r6,  r12, lsr #31
                    adcs r7, r7,  r12

                    pop {pc}
                ",
                options(raw)
            )
        } else {
            naked_asm!(
                "
                    push {r2, lr}
                    // frame push {lr}
                    // frame address sp, 8

                    sub sp, #28
                    // frame address sp,36
                    ldm r2, {r2, r3, r4, r5}

                    ldm r1!, {r0, r10, lr}
                    umull r6, r11, r2, r0

                    umull r7, r12, r3, r0
                    umaal r7, r11, r2, r10

                    push {r6, r7}
                    // frame address sp,44

                    umull r8, r6,  r4, r0
                    umaal r8, r11, r3, r10

                    umull r9, r7,  r5, r0
                    umaal r9, r11, r4, r10

                    umaal r11, r7, r5, r10

                    umaal r8,  r12, r2, lr
                    umaal r9,  r12, r3, lr
                    umaal r11, r12, r4, lr
                    umaal r12, r7,  r5, lr

                    ldm r1!, {r0, r10, lr}

                    umaal r9,  r6, r2, r0
                    umaal r11, r6, r3, r0
                    umaal r12, r6, r4, r0
                    umaal r6,  r7, r5, r0

                    strd r8, r9, [sp, #8]

                    mov r9, #0
                    umaal r11, r9, r2, r10
                    umaal r12, r9, r3, r10
                    umaal r6,  r9, r4, r10
                    umaal r7,  r9, r5, r10

                    mov r10, #0
                    umaal r12, r10, r2, lr
                    umaal r6,  r10, r3, lr
                    umaal r7,  r10, r4, lr
                    umaal r9,  r10, r5, lr

                    ldr r8, [r1], #4
                    mov lr, #0
                    umaal lr,  r6, r2, r8
                    umaal r7,  r6, r3, r8
                    umaal r9,  r6, r4, r8
                    umaal r10, r6, r5, r8

                    //_ _ _ _ _ 6 10 9| 7 | lr 12 11 _ _ _ _

                    ldr r8, [r1], #-28
                    mov r0, #0
                    umaal r7,  r0, r2, r8
                    umaal r9,  r0, r3, r8
                    umaal r10, r0, r4, r8
                    umaal r6,  r0, r5, r8

                    push {r0}
                    // frame address sp,48

                    //_ _ _ _ s 6 10 9| 7 | lr 12 11 _ _ _ _

                    ldr r2, [sp, #40]
                    adds r2, r2, #16
                    ldm r2, {r2, r3, r4, r5}

                    ldr r8, [r1], #4
                    mov r0, #0
                    umaal r11, r0, r2, r8
                    str r11, [sp, #16+4]
                    umaal r12, r0, r3, r8
                    umaal lr,  r0, r4, r8
                    umaal r0,  r7, r5, r8 // 7=carry for 9

                    //_ _ _ _ s 6 10 9+7| 0 | lr 12 _ _ _ _ _

                    ldr r8, [r1], #4
                    mov r11, #0
                    umaal r12, r11, r2, r8
                    str r12, [sp, #20+4]
                    umaal lr,  r11, r3, r8
                    umaal r0,  r11, r4, r8
                    umaal r11, r7,  r5, r8 // 7=carry for 10

                    //_ _ _ _ s 6 10+7 9+11| 0 | lr _ _ _ _ _ _

                    ldr r8, [r1], #4
                    mov r12, #0
                    umaal lr, r12, r2, r8
                    str lr, [sp, #24+4]
                    umaal r0,  r12, r3, r8
                    umaal r11, r12, r4, r8
                    umaal r10, r12, r5, r8 // 12=carry for 6

                    //_ _ _ _ s 6+12 10+7 9+11| 0 | _ _ _ _ _ _ _

                    ldr r8, [r1], #4
                    mov lr, #0
                    umaal r0, lr, r2, r8
                    str r0, [sp, #28+4]
                    umaal r11, lr, r3, r8
                    umaal r10, lr, r4, r8
                    umaal r6,  lr, r5, r8 // lr=carry for saved

                    //_ _ _ _ s+lr 6+12 10+7 9+11| _ | _ _ _ _ _ _ _

                    ldm r1!, {r0, r8}
                    umaal r11, r9, r2, r0
                    str r11, [sp, #32+4]
                    umaal r9, r10, r3, r0
                    umaal r10, r6, r4, r0
                    pop {r11}
                    //frame address sp,44
                    umaal r11, r6, r5, r0 // 6=carry for next

                    //_ _ _ 6 11+lr 10+12 9+7 _ | _ | _ _ _ _ _ _ _

                    umaal r9,  r7, r2, r8
                    umaal r10, r7, r3, r8
                    umaal r11, r7, r4, r8
                    umaal r6,  r7, r5, r8

                    ldm r1!, {r0, r8}
                    umaal r10, r12, r2, r0
                    umaal r11, r12, r3, r0
                    umaal r6,  r12, r4, r0
                    umaal r7,  r12, r5, r0

                    umaal r11, lr,  r2, r8
                    umaal lr,  r6,  r3, r8
                    umaal r6,  r7,  r4, r8
                    umaal r7,  r12, r5, r8

                    // 12 7 6 lr 11 10 9 stack*9
                    push {r6, r7, r12}
                    // frame address sp,56
                    add r7, sp, #12
                    ldm r7, {r0-r8}

                    mov r12, #0

                    adds r3, r0
                    adcs r4,  r4,  r1
                    adcs r5,  r5,  r2
                    adcs r6,  r6,  r0
                    adcs r7,  r7,  r1
                    adcs r8,  r8,  r0
                    adcs r9,  r9,  r1
                    adcs r10, r10, #0
                    adcs r11, r11, #0
                    adcs r12, r12, #0

                    adds r6, r3
                    adcs r7,  r7,  r4 // r4 instead of 0
                    adcs r8,  r8,  r2
                    adcs r9,  r9,  r3
                    adcs r10, r10, r2
                    adcs r11, r11, r3
                    adcs r12, r12, #0

                    subs r7, r0
                    sbcs r8,  r8,  r1
                    sbcs r9,  r9,  r2
                    sbcs r10, r10, r3
                    sbcs r11, r11, #0
                    sbcs r12, r12, #0 // r12 is between 0 and 2

                    pop {r1-r3}
                    // frame address sp,44

                    adds r0, lr, r12
                    adcs r1, r1, #0
                    mov r12, #0
                    adcs r12, r12, #0

                    //adds r7,r4 (added above instead)
                    adcs r8,  r8,  r5
                    adcs r9,  r9,  r6
                    adcs r10, r10, r4
                    adcs r11, r11, r5
                    adcs r0,  r0,  r4
                    adcs r1,  r1,  r5
                    adcs r2,  r2,  r12
                    adcs r3,  r3,  #0
                    mov r12, #0
                    adcs r12, r12, #0

                    adcs r10, r10, r7
                    adcs r11, r11, #0
                    adcs r0,  r0,  r6
                    adcs r1,  r1,  r7
                    adcs r2,  r2,  r6
                    adcs r3,  r3,  r7
                    adcs r12, r12, #0

                    subs r11, r4
                    sbcs r0,  r0,  r5
                    sbcs r1,  r1,  r6
                    sbcs r2,  r2,  r7
                    sbcs r3,  r3,  #0
                    sbcs r12, r12, #0

                    // now (T + mN) / R is
                    // 8 9 10 11 0 1 2 3 12 (lsb -> msb)

                    subs r8,  r8,  #0xffffffff
                    sbcs r9,  r9,  #0xffffffff
                    sbcs r10, r10, #0xffffffff
                    sbcs r11, r11, #0
                    sbcs r4,  r0,  #0
                    sbcs r5,  r1,  #0
                    sbcs r6,  r2,  #1
                    sbcs r7,  r3,  #0xffffffff
                    sbc  r12,r12,  #0

                    adds r0, r8,  r12
                    adcs r1, r9,  r12
                    adcs r2, r10, r12
                    adcs r3, r11, #0
                    adcs r4, r4,  #0
                    adcs r5, r5,  #0
                    adcs r6, r6,  r12, lsr #31
                    adcs r7, r7,  r12

                    add sp,sp,#40
                    // frame address sp,4

                    pop {pc}
                ",
                options(raw)
            )
        }
    }
}

/// Given inputs `a` and `b`, calculate `a * b mod n`, where `n` is the P256 order.
///
/// # Inputs
/// `r0` shall contain a valid `*mut [u32; 8]`.
///
/// `r1` shall contain `a`, a valid `*const [u32; 8]`.
///
/// `r2` shall contain `b`, a valid `*const [u32; 8]`.
///
/// `r0` and `r1` and/or `r2` may overlap.
///
/// # Returns
/// On return, the dereference of the input value of `r0` will contain the result of the computation.
///
/// # Safety
/// The caller must guarantee that `res`, `a` and `b` are valid for the duration
/// of the function call, and that `res` is valid for writes.
///
/// > **Note**: This function adheres to the ARM calling convention.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub(in crate::sys) unsafe extern "C" fn P256_mul_mod_n(
    res: *mut [u32; 8],
    a: *const [u32; 8],
    b: *const [u32; 8],
) {
    naked_asm!(
        "
            movs r3,#0
            push {{r3-r10, lr}}
            // frame push {{r4-r10, lr}}
            // frame address sp, 36

            mov r4, r0

            ldm r1,{{r1, r3, r5-r10}}
            push {{r1, r3, r5-r10}}
            // frame address sp, 68

            movs r1, #0
            push {{r1}}
            // frame address sp,72
            ldm r2, {{r1, r3, r5-r10}}
            push {{r1, r3, r5-r10}}
            // frame address sp,104

            sub sp, #72
            //frame address sp,176

            mov r0, sp
            add r1, sp, #72
            add r2, sp, #108
            bl {mul288x288} // just reuse the 288x288-bit multiplier rather than also writing a 256x256

            mov r0,r4
            mov r1,sp
            bl {P256_reduce_mod_n_64bytes}

            add sp,#144
            //frame address sp,32
            pop {{r4-r10,pc}}
        ",
        mul288x288 = sym super::mul288x288,
        P256_reduce_mod_n_64bytes = sym super::P256_reduce_mod_n_64bytes,
    )
}
