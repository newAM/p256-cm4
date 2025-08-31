use core::arch::naked_asm;

/// For input `A*R mod p`, computes `A^2*R mod p`.
///
/// # Inputs
/// `r0` through `r7` shall contain `A*R mod p`. TODO: figure out ordering. Montgomery?
///
/// # Return
/// On return, `r0` through `r7` will contain `A^2*R mod p`.
///
/// All other registers are clobbered.
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn P256_sqrmod() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "use-mul-for-sqr")] {
            naked_asm!(
                "
                    push {{r0-r7,lr}}
                    // frame push {{lr}}
                    // frame address sp,36
                    mov r1, sp
                    mov r2, sp
                    bl {P256_mulmod}
                    add sp, sp, #32
                    // frame address sp,4
                    pop {{pc}}
                ",
                P256_mulmod = sym super::P256_mulmod,
            )
        } else if #[cfg(feature = "no-fpu")] {
            naked_asm!(
                "
                    push {lr}
                    // frame push {lr}

                    // mul 01, 00
                    umull r9,  r10, r0,r0
                    umull r11, r12, r0,r1
                    adds r11, r11, r11
                    mov lr, #0
                    umaal r10, r11, lr, lr

                    // r10 r9 done
                    // r12 carry for 3rd before col
                    // r11+C carry for 3rd final col

                    push {r9, r10}
                    // frame address sp,12

                    // mul 02, 11
                    mov r9,#0
                    umaal r9, r12, r0, r2
                    adcs r9, r9, r9
                    umaal r9, r11, r1, r1

                    // r9 done (3rd col)
                    // r12 carry for 4th before col
                    // r11+C carry for 4th final col

                    push {r9}
                    // frame address sp,16

                    // mul 03, 12
                    umull r9, r10, r0, r3
                    umaal r9, r12, r1, r2
                    adcs r9, r9, r9
                    umaal r9, r11, lr, lr

                    // r9 done (4th col)
                    // r10+r12 carry for 5th before col
                    // r11+C carry for 5th final col

                    push {r9}
                    // frame address sp,20

                    // mul 04, 13, 22
                    mov r9, #0
                    umaal r9, r10, r0, r4
                    umaal r9, r12, r1, r3
                    adcs r9, r9, r9
                    umaal r9, r11, r2 ,r2

                    // r9 done (5th col)
                    // r10+r12 carry for 6th before col
                    // r11+C carry for 6th final col

                    push {r9}
                    // frame address sp,24

                    // mul 05, 14, 23
                    umull r9 ,r8,  r0, r5
                    umaal r9 ,r10, r1, r4
                    umaal r9 ,r12, r2, r3
                    adcs r9, r9, r9
                    umaal r9, r11, lr, lr

                    // r9 done (6th col)
                    // r10+r12+r8 carry for 7th before col
                    // r11+C carry for 7th final col

                    push {r9}
                    // frame address sp,28

                    // mul 06, 15, 24, 33
                    mov r9, #0
                    umaal r9, r8,  r1, r5
                    umaal r9, r12, r2, r4
                    umaal r9, r10, r0, r6
                    adcs r9, r9, r9
                    umaal r9, r11, r3, r3

                    // r9 done (7th col)
                    // r8+r10+r12 carry for 8th before col
                    // r11+C carry for 8th final col

                    push {r9}
                    // frame address sp,32

                    //mul 07, 16, 25, 34
                    umull r9, r0,  r0, r7
                    umaal r9, r10, r1, r6
                    umaal r9, r12, r2, r5
                    umaal r9, r8,  r3, r4
                    adcs r9, r9, r9
                    umaal r9, r11, lr, lr

                    // r9 done (8th col)
                    // r0+r8+r10+r12 carry for 9th before col
                    // r11+C carry for 9th final col

                    // mul 17, 26, 35, 44
                    umaal r0, r8,  r1, r7 // r1 is now dead
                    umaal r0, r10, r2, r6
                    // pop {r1}
                    // //frame address sp,32
                    umaal r0, r12, r3, r5
                    adcs r0, r0, r0
                    umaal r11, r0, r4, r4

                    // r11 done (9th col)
                    // r8+r10+r12 carry for 10th before col
                    // r0+C carry for 10th final col

                    //mul 27, 36, 45
                    umaal r12, r8,  r2, r7 // r2 is now dead
                    umaal r12, r10, r3, r6
                    movs r2, #0
                    umaal r12, r2, r4, r5
                    adcs r1, r12, r12
                    umaal r0, r1, lr, lr

                    // r0 done (10th col)
                    // r8+r10+r2 carry for 11th before col
                    // r1+C carry for 11th final col

                    // mul 37, 46, 55
                    umaal r2, r8,  r3, r7 // r3 is now dead
                    umaal r2, r10, r4, r6
                    adcs r2, r2, r2
                    umaal r1, r2, r5, r5

                    // r1 done (11th col)
                    // r8+r10 carry for 12th before col
                    // r2+C carry for 12th final col

                    // mul 47, 56
                    movs r3, #0
                    umaal r3, r8, r4, r7 // r4 is now dead
                    umaal r3 ,r10, r5, r6
                    adcs r3, r3, r3
                    umaal r2, r3, lr, lr

                    // r2 done (12th col)
                    // r8+r10 carry for 13th before col
                    // r3+C carry for 13th final col

                    //mul 57, 66
                    umaal r8, r10, r5, r7 // r5 is now dead
                    adcs r8, r8, r8
                    umaal r3, r8, r6, r6

                    // r3 done (13th col)
                    // r10 carry for 14th before col
                    // r8+C carry for 14th final col

                    // mul 67
                    umull r4, r5,  lr, lr // set 0
                    umaal r4, r10, r6, r7
                    adcs r4, r4, r4
                    umaal r4, r8, lr, lr

                    // r4 done (14th col)
                    // r10 carry for 15th before col
                    // r8+C carry for 15th final col

                    // mul 77
                    adcs r10, r10, r10
                    umaal r8, r10, r7, r7
                    adcs r10, r10, lr

                    // r8 done (15th col)
                    // r10 done (16th col)

                    // msb -> lsb: r10 r8 r4 r3 r2 r1 r0 r11 r9 sp sp+4 sp+8 sp+12 sp+16 sp+24 sp+20
                    // now do reduction

                    push {r4, r8, r10}
                    // frame address sp,44
                    add r4, sp, #12
                    ldm r4, {r4-r8, r10, r12}
                    // lr is already 0
                    X0 .req r10
                    X1 .req r12
                    X2 .req r8
                    X3 .req r7
                    X4 .req r6
                    X5 .req r5
                    X6 .req r4
                    X7 .req r9
                    X8 .req r11
                    X9 .req r0
                    X10 .req r1
                    X11 .req r2
                    X12 .req r3

                    X13 .req r7
                    X14 .req r8
                    X15 .req r12

                    adcs X3,  X3,  X0
                    adcs X4,  X4,  X1
                    adcs X5,  X5,  X2
                    adcs X6,  X6,  X0
                    adcs X7,  X7,  X1
                    adcs X8,  X8,  X0
                    adcs X9,  X9,  X1
                    adcs X10, X10, #0
                    adcs X11, X11, #0
                    adcs lr, lr, #0

                    adds X6, X3
                    adcs X7,  X7,  X4 // X4 instead of 0
                    adcs X8,  X8,  X2
                    adcs X9,  X9,  X3
                    adcs X10, X10, X2
                    adcs X11, X11, X3
                    adcs lr,  lr,  #0

                    subs X7, X0
                    sbcs X8,  X8,  X1
                    sbcs X9,  X9,  X2
                    sbcs X10, X10, X3
                    sbcs X11, X11, #0
                    sbcs lr, lr, #0 // lr is between 0 and 2

                    pop {X13, X14, X15}
                    // frame address sp,32

                    adds X0,  X12, lr
                    adcs X13, X13, #0
                    mov lr, #0
                    adcs lr, lr,# 0

                    //adds X7,X4 (added above instead)
                    adcs X8,  X8,  X5
                    adcs X9,  X9,  X6
                    adcs X10, X10, X4
                    adcs X11, X11, X5
                    adcs X0,  X0,  X4
                    adcs X13, X13, X5
                    adcs X14, X14, lr
                    adcs X15, X15, #0
                    mov lr, #0
                    adcs lr, lr, #0

                    adcs X10, X10, X7
                    adcs X11, X11, #0
                    adcs X0,  X0,  X6
                    adcs X13, X13, X7
                    adcs X14, X14, X6
                    adcs X15, X15, X7
                    adcs lr, lr, #0

                    subs X11, X4
                    sbcs X0,  X0,  X5
                    sbcs X13, X13, X6
                    sbcs X14, X14, X7
                    sbcs X15, X15, #0
                    sbcs lr, lr, #0

                    // now (T + mN) / R is
                    // X8 X9 X10 X11 X0 X13 X14 X15 lr (lsb -> msb)
                    // r11 r0 r1 r2 r10 r7 r8 r12 lr

                    subs r11, r11, #0xffffffff
                    sbcs r9,  r0,  #0xffffffff
                    sbcs r4,  r1,  #0xffffffff
                    sbcs r3,  r2,  #0
                    sbcs r6,  r10, #0
                    sbcs r5,  r7,  #0
                    sbcs r10, r8,  #1
                    sbcs r8,  r12, #0xffffffff
                    sbcs r7,  lr,  #0

                    adds r0, r11, r7
                    adcs r1, r9,  r7
                    adcs r2, r4,  r7
                    adcs r3, r3,  #0
                    adcs r4, r6,  #0
                    adcs r5, r5,  #0
                    adcs r6, r10, r7, lsr #31
                    adcs r7, r8,  r7

                    add sp,#28
                    // frame address sp,4
                    pop {pc}
                ",
                options(raw)
            )
        } else {
            naked_asm!(
                "
                    push {lr}
                    // frame push {lr}

                    // mul 01, 00
                    umull r9,  r10, r0, r0
                    umull r11, r12, r0, r1
                    adds r11, r11, r11
                    mov lr, #0
                    umaal r10, r11, lr, lr

                    // r9 r10 done
                    // r12 carry for 3rd before col
                    // r11+C carry for 3rd final col

                    vmov s0, s1, r9, r10

                    // mul 02, 11
                    mov r8, #0
                    umaal r8, r12, r0, r2
                    adcs r8, r8, r8
                    umaal r8, r11, r1, r1

                    // r8 done (3rd col)
                    // r12 carry for 4th before col
                    // r11+C carry for 4th final col

                    // mul 03, 12
                    umull r9, r10, r0, r3
                    umaal r9, r12, r1, r2
                    adcs r9, r9, r9
                    umaal r9, r11, lr, lr

                    // r9 done (4th col)
                    // r10+r12 carry for 5th before col
                    // r11+C carry for 5th final col

                    vmov s2, s3, r8, r9

                    // mul 04, 13, 22
                    mov r9, #0
                    umaal r9, r10, r0, r4
                    umaal r9, r12, r1, r3
                    adcs r9, r9, r9
                    umaal r9, r11, r2, r2

                    // r9 done (5th col)
                    // r10+r12 carry for 6th before col
                    // r11+C carry for 6th final col

                    vmov s4, r9

                    // mul 05, 14, 23
                    umull r9, r8,  r0, r5
                    umaal r9, r10, r1, r4
                    umaal r9, r12, r2, r3
                    adcs r9, r9, r9
                    umaal r9, r11, lr, lr

                    // r9 done (6th col)
                    // r10+r12+r8 carry for 7th before col
                    // r11+C carry for 7th final col

                    vmov s5, r9

                    //mul 06, 15, 24, 33
                    mov r9, #0
                    umaal r9, r8,  r1, r5
                    umaal r9, r12, r2, r4
                    umaal r9, r10, r0, r6
                    adcs r9, r9, r9
                    umaal r9, r11, r3, r3

                    // r9 done (7th col)
                    // r8+r10+r12 carry for 8th before col
                    // r11+C carry for 8th final col

                    vmov s6, r9

                    //mul 07, 16, 25, 34
                    umull r0, r9,  r0, r7
                    umaal r0, r10, r1, r6
                    umaal r0, r12, r2, r5
                    umaal r0, r8,  r3, r4
                    adcs r0, r0, r0
                    umaal r0, r11, lr, lr

                    // r0 done (8th col)
                    // r9+r8+r10+r12 carry for 9th before col
                    // r11+C carry for 9th final col

                    // mul 17, 26, 35, 44
                    umaal r9,  r8,  r1, r7 //r1 is now dead
                    umaal r9,  r10, r2, r6
                    umaal r12, r9,  r3, r5
                    adcs r12, r12, r12
                    umaal r11, r12, r4, r4

                    // r11 done (9th col)
                    // r8+r10+r9 carry for 10th before col
                    // r12+C carry for 10th final col

                    // mul 27, 36, 45
                    umaal r9,  r8, r2, r7 // r2 is now dead
                    umaal r10, r9, r3, r6
                    movs r2, #0
                    umaal r10, r2, r4, r5
                    adcs r10, r10, r10
                    umaal r12, r10, lr, lr

                    // r12 done (10th col)
                    // r8+r9+r2 carry for 11th before col
                    // r10+C carry for 11th final col

                    // mul 37, 46, 55
                    umaal r2, r8, r3, r7 // r3 is now dead
                    umaal r9, r2, r4, r6
                    adcs r9, r9, r9
                    umaal r10, r9, r5, r5

                    // r10 done (11th col)
                    // r8+r2 carry for 12th before col
                    // r9+C carry for 12th final col

                    // mul 47, 56
                    movs r3, #0
                    umaal r3, r8, r4, r7 // r4 is now dead
                    umaal r3, r2, r5, r6
                    adcs r3, r3, r3
                    umaal r9, r3, lr, lr

                    // r9 done (12th col)
                    // r8+r2 carry for 13th before col
                    // r3+C carry for 13th final col

                    // mul 57, 66
                    umaal r8, r2, r5, r7 // r5 is now dead
                    adcs r8, r8, r8
                    umaal r3, r8, r6, r6

                    // r3 done (13th col)
                    // r2 carry for 14th before col
                    // r8+C carry for 14th final col

                    // mul 67
                    umull r4, r5, lr, lr // set 0
                    umaal r4, r2, r6, r7
                    adcs r4, r4, r4
                    umaal r4, r8, lr, lr

                    // r4 done (14th col)
                    // r2 carry for 15th before col
                    // r8+C carry for 15th final col

                    // mul 77
                    adcs r2, r2, r2
                    umaal r8, r2, r7, r7
                    adcs r2, r2, lr

                    // r8 done (15th col)
                    // r2 done (16th col)

                    // msb -> lsb: r2 r8 r4 r3 r9 r10 r12 r11 r0 s6 s5 s4 s3 s2 s1 s0
                    // lr: 0
                    // now do reduction

                    vmov s13, s14, r4, r8
                    vmov s15, r2 //s15

                    vmov r1, r2, s0, s1
                    vmov r8, r7, s2, s3
                    vmov r6, r5, s4, s5
                    vmov r4, s6
                    // lr is already 0
                    X0 .req r1
                    X1 .req r2
                    X2 .req r8
                    X3 .req r7
                    X4 .req r6
                    X5 .req r5
                    X6 .req r4
                    X7 .req r0
                    X8 .req r11
                    X9 .req r12
                    X10 .req r10
                    X11 .req r9
                    X12 .req r3

                    X13 .req r7
                    X14 .req r8
                    X15 .req r2

                    adcs X3,  X3,  X0
                    adcs X4,  X4,  X1
                    adcs X5,  X5,  X2
                    adcs X6,  X6,  X0
                    adcs X7,  X7,  X1
                    adcs X8,  X8,  X0
                    adcs X9,  X9,  X1
                    adcs X10, X10, #0
                    adcs X11, X11, #0
                    adcs lr,  lr,  #0

                    adds X6, X3
                    adcs X7, X7, X4 // X4 instead of 0
                    adcs X8,  X8,  X2
                    adcs X9,  X9,  X3
                    adcs X10, X10, X2
                    adcs X11, X11, X3
                    adcs lr,  lr,  #0

                    subs X7,  X0
                    sbcs X8,  X8,  X1
                    sbcs X9,  X9,  X2
                    sbcs X10, X10, X3
                    sbcs X11, X11, #0
                    sbcs lr,  lr,  #0 // lr is between 0 and 2

                    vmov X13, X14, s13, s14
                    vmov X15, s15

                    adds X0,  X12, lr
                    adcs X13, X13, #0
                    mov lr, #0
                    adcs lr, lr, #0

                    //adds X7,X4 (added above instead)
                    adcs X8,  X8,  X5
                    adcs X9,  X9,  X6
                    adcs X10, X10, X4
                    adcs X11, X11, X5
                    adcs X0,  X0,  X4
                    adcs X13, X13, X5
                    adcs X14, X14, lr
                    adcs X15, X15, #0
                    mov lr, #0
                    adcs lr, lr, #0

                    adcs X10, X10, X7
                    adcs X11, X11, #0
                    adcs X0,  X0,  X6
                    adcs X13, X13, X7
                    adcs X14, X14, X6
                    adcs X15, X15, X7
                    adcs lr, lr, #0

                    subs X11, X4
                    sbcs X0,  X0,  X5
                    sbcs X13, X13, X6
                    sbcs X14, X14, X7
                    sbcs X15, X15, #0
                    sbcs lr,  lr,  #0

                    // now (T + mN) / R is
                    // X8 X9 X10 X11 X0 X13 X14 X15 lr (lsb -> msb)
                    // r11 r12 r10 r9 r1 r7 r8 r2 lr

                    subs r0,  r11, #0xffffffff
                    sbcs r12, r12, #0xffffffff
                    sbcs r4,  r10, #0xffffffff
                    sbcs r9,  r9,  #0
                    sbcs r6,  r1,  #0
                    sbcs r5,  r7,  #0
                    sbcs r10, r8,  #1
                    sbcs r8,  r2,  #0xffffffff
                    sbcs r7,  lr,  #0

                    adds r0, r0,  r7
                    adcs r1, r12, r7
                    adcs r2, r4,  r7
                    adcs r3, r9,  #0
                    adcs r4, r6,  #0
                    adcs r5, r5,  #0
                    adcs r6, r10, r7, lsr #31
                    adcs r7, r8,  r7

                    pop {pc}
                ",
                options(raw)
            )
        }
    }
}
