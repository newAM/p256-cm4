use core::arch::naked_asm;

use crate::sys::asm::montgomery::Montgomery;

/// Perform the last step of verifying a P256 signature.
///
/// This function checks that `r === x (mod n)`.
///
/// # Inputs
/// `r0` shall contain `r`, a valid `*const [u32; 8]`.
///
/// `r1` shall contain a valid (`*const [Montgomery; 3]`)[super::Montgomery], the result of performing a double scalarmult in jacobian form.
///
/// # Return
/// On return, `r0` will contain `1` if the check passes, and `0` otherwise.
///
/// # Safety
/// The caller must guarantee that `r` and `x` are valid for the duration of the function
/// call.
///
/// > **Note**: This function adheres to the ARM calling convention.
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".p256-cortex-m4")]
pub(in crate::sys) unsafe extern "C" fn P256_verify_last_step(
    r: *const [u32; 8],
    x: *const [Montgomery; 3],
) -> bool {
    naked_asm!(
        "
            push {{r0, r1, r4-r11, lr}}
            // frame push {{r0, r1, r4-r11, lr}}
            // frame address sp,44
            sub sp,#32
            // frame address sp,76

            // Instead of doing an expensive field inversion and checking r = (X/Z^2 % p) (mod n),
            // accept the signature iff r*Z^2 % p = X OR (r+n<p AND (r+n)*Z^2 % p = X).
            // Proof that this is correct:
            //   if we use the standard approach, that would mean we check that
            //   r = (X/Z^2 % p) (mod n)
            //   which is the same as r+k*n = (X/Z^2 % p) for any integer k,
            //   but since the RHS is less than p and 2n > p, we only need to check for k=0,1
            //   which means checking r = (X/Z^2 % p) OR r+n = (X/Z^2 % p)
            //   For r = (X/Z^2 % p) we have that r < p and so we can instead check r*Z^2 % p = X
            //   For r+n = (X/Z^2 % p) we must first check that r+n < p and can then check (r+n)*Z^2 % p = X
            //
            // Note that since p-n is around sqrt(n), it is extremely unlikely that r+n<p
            //
            // Note that X and Z are in Montgomery form but not r,
            // so we must convert r to Montgomery form when it's time to do the multiplications

            // Calculate Z^2
            add r1, #64
            ldm r1, {{r0-r7}}
            bl {P256_sqrmod}
            push {{r0-r7}}
            // frame address sp,108

            // Check if Z^2 if 0, if so reject
            orrs r0, r1
            orrs r0, r2
            orrs r0, r3
            orrs r0, r4
            orrs r0, r5
            orrs r0, r6
            orrs r0, r7
            beq 0f

            // Convert r to Montgomery form
            ldr r1, [sp, #64]
        2:
            add r0, sp, #32
            bl {P256_to_montgomery}

            // Calculate r*Z^2
            add r1, sp, #32
            mov r2, sp
            bl {P256_mulmod}

            // Now we will check if r*Z^2 = X
            ldr r8, [sp, #68]
            ldm r8!, {{r9-r12}}
            eors r0, r9
            ittt eq
            eorseq r1, r10
            eorseq r2, r11
            eorseq r3, r12
            ldm r8!, {{r9-r12}}
            itttt eq
            eorseq r4, r9
            eorseq r5, r10
            eorseq r6, r11
            eorseq r7, r12
            mov r0, #1
            beq 1f

            // The check may fail if r < p-n, so also check for r' = r+n
            adr r0, {P256_order}
            ldm r0, {{r8-r11}}
            ldr r0, [sp, #64]
            cbz r0, 0f // if we already tried once, abort
            ldm r0, {{r0-r7}}
            adds r0, r8
            adcs r1, r1, r9
            adcs r2, r2, r10
            adcs r3, r3, r11
            adcs r4, r4, #0xffffffff
            adcs r5, r5, #0xffffffff
            adcs r6, r6, #0
            adcs r7, r7, #0xffffffff
            bcs 0f // reject if r+n >= 2^256 (which is >= p)

            subs r8, r0, #0xffffffff
            sbcs r8, r1, #0xffffffff
            sbcs r8, r2, #0xffffffff
            sbcs r8, r3, #0
            sbcs r8, r4, #0
            sbcs r8, r5, #0
            sbcs r8, r6, #1
            sbcs r8, r7, #0xffffffff
            bcs 0f // reject if r+n >= p

            add r8, sp, #32
            stm r8, {{r0-r7}}
            movs r2, #0
            str r2, [sp, #64] // set r variable to NULL to avoid yet another try

            mov r1,r8
            b 2b

        0:
            movs r0,#0
        1:
            add sp,#72
            // frame address sp,36
            pop {{r4-r11,pc}}
        ",
        P256_mulmod = sym super::P256_mulmod,
        P256_sqrmod = sym super::P256_sqrmod,
        P256_to_montgomery = sym super::P256_to_montgomery,
        P256_order = sym super::P256_ORDER,
    )
}
