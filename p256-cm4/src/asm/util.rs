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
