use core::arch::naked_asm;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Montgomery([u32; 8]);

impl Montgomery {
    pub const fn zero() -> Self {
        Self([0u32; _])
    }
}

impl core::ops::Index<usize> for Montgomery {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Default for Montgomery {
    fn default() -> Self {
        Self::zero()
    }
}

/// Convert a number from normal representation to montgomery representation.
///
/// # Inputs
/// Register `r0` shall contain a valid [`*mut Montgomery`](Montgomery).
///
/// Register `r1` shall contain a valid `*const [u32; 8]`, pointing to the 256-bit number to be converted.
///
/// The pointers in `r0` and `r1` are allowed to be overlapping.
///
/// # Return
/// On return, the dereference of the input value of `r0` shall contain the result of the computation.
///
/// > **Note**: `r0` will be overriden during the execution of this function (it is callee-saved).
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P256_to_montgomery(a: *mut Montgomery, aR: *const [u32; 8]) {
    naked_asm!(
        "
            push {{r0, r4-r11, lr}}
            // frame push {{r4-r11, lr}}
            // frame address sp, 40
            adr r2, 0f
            bl {P256_mulmod}
            pop {{r8}}
            // frame address sp ,36
            stm r8, {{r0-r7}}
            pop {{r4-r11,pc}}

            .align 2
        0: // R2_mod_p
            .word 3
            .word 0
            .word 0xffffffff
            .word 0xfffffffb
            .word 0xfffffffe
            .word 0xffffffff
            .word 0xfffffffd
            .word 4
        ",
        P256_mulmod = sym super::P256_mulmod,
    )
}
