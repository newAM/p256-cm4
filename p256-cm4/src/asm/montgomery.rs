use core::arch::naked_asm;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Montgomery(pub(in crate::sys) [u32; 8]);

impl Montgomery {
    /// Create a new [`Montgomery`] with the given value.
    ///
    /// For conversion from little-endian integers, see the
    /// [`From`] impls for this type.
    pub(crate) const fn new(value: [u32; 8]) -> Self {
        Self(value)
    }

    pub const fn one() -> Self {
        Self([1, 0, 0, 0xffffffff, 0xffffffff, 0xffffffff, 0xfffffffe, 0])
    }

    pub const fn zero() -> Self {
        Self([0u32; _])
    }
}

/// Convert a number from normal representation to montgomery representation.
///
/// # Inputs
/// Register `r0` shall contain a valid [`*mut Montgomery`](Montgomery).
///
/// Register `r1` shall contain a valid `*const [u32; 8]`, pointing to the 256-bit number to be converted.
///
/// The pointers in `r0` and `r1` may overlap.
///
/// # Return
/// On return, the dereference of the input value of `r0` shall contain the result of the computation.
///
/// # Safety
/// The caller must guarantee that `a` and `aR` are valid for the duration of the function call,
/// and that `a` is valid for writes.
///
/// > **Note**: This function adheres to the ARM calling convention.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub(in crate::sys) unsafe extern "C" fn P256_to_montgomery(
    a: *mut Montgomery,
    aR: *const [u32; 8],
) {
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

/// Convert a number from normal representation to montgomery representation.
///
/// # Inputs
/// `r0` shall contain a valid `*mut [u32; 8]`.
///
/// `r1` shall contain a valid [`*cont Montgomery`](Montgomery), the number to be converted.
///
/// The pointers in `r0` and `r1` may overlap.
///
/// # Return
/// On return, the dereference of the input value of `r0` shall contain the result of the computation.
///
/// # Safety
/// The caller must guarantee that `a` and `aR` are valid for the duration of the function call,
/// and that `a` is valid for writes.
///
/// > **Note**: This function adheres to the ARM calling convention.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub(in crate::sys) unsafe extern "C" fn P256_from_montgomery(
    a: *mut [u32; 8],
    aR: *const Montgomery,
) {
    naked_asm!(
        "
            push {{r0,r4-r11,lr}}
            // frame push {{r4-r11, lr}}
            // frame address sp, 40
            // Construct the number 1 on the stack
            movs r2, #0
            movs r3, #0
            push {{r2-r3}}
            // frame address sp, 48
            push {{r2-r3}}
            // frame address sp, 56
            push {{r2-r3}}
            // frame address sp, 64
            movs r2,#1
            push {{r2-r3}}
            // frame address sp, 72
            mov r2,sp
            bl {P256_mulmod}
            add sp,#32
            // frame address sp, 40
            pop {{r8}}
            // frame address sp, 36
            stm r8, {{r0-r7}}
            pop {{r4-r11,pc}}
        ",
        P256_mulmod = sym super::P256_mulmod,
    )
}
