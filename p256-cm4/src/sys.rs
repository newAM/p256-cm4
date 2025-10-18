#[path = "./asm/mod.rs"]
#[cfg(target_arch = "arm")]
pub(crate) mod asm;

pub use asm::montgomery::Montgomery;

impl Montgomery {
    /// Set the contents of `self` to the montgomery representation
    /// of the little-endian integer `normal`.
    pub fn read(&mut self, normal: &[u32; 8]) {
        // SAFETY: `normal` and `mont` are valid for the duration of
        // the function call, and `mont` is valid for writes.
        unsafe { asm::montgomery::P256_to_montgomery(self, normal) };
    }

    /// Write the little-endian integer representation of `self`
    /// to `normal`.
    pub fn write(&self, normal: &mut [u32; 8]) {
        // SAFETY: `normal` and `mont` are valid for the duration of
        // the function call, and `normal` is valid for writes.
        unsafe { asm::montgomery::P256_from_montgomery(normal, self) };
    }
}

impl From<[u32; 8]> for Montgomery {
    fn from(mut normal: [u32; 8]) -> Self {
        // SAFETY: `normal` is valid for the duration of the function
        // call, and is valid for writes. The written-to and read-from
        // pointers passed to `P256_to_montgomery` are allowed to overlap.
        unsafe { asm::montgomery::P256_to_montgomery(&raw mut normal as _, &raw const normal) };
        Self(normal)
    }
}

impl From<Montgomery> for [u32; 8] {
    fn from(value: Montgomery) -> Self {
        let mut array = value.0;

        // SAFETY: `array` is valid for the duration of the function call
        // call, and is valid for writes. The written-to and read-from
        // pointers passed to `P256_from_montgomery` are allowed to overlap.
        unsafe { asm::montgomery::P256_from_montgomery(&raw mut array, &raw const array as _) };

        array
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

/// Checks that the argument, as little-endian integer,
/// is a reduced non-zero element of the scalar field.
///
/// In other words, that it is in the range `1..=n-1`,
/// where `n = 2^256 - 2^224 + 2^192 - 0x4319055258e8617b0c46353d039cdaaf`.
#[inline(always)]
#[must_use]
pub fn check_range_n(a: &[u32; 8]) -> bool {
    // SAFETY: `a` is valid for the duration of the call.
    unsafe { asm::P256_check_range_n(a) }
}

/// Checks that the argument, as little-endian integer,
/// is a reduced element of the base field.
///
/// In other words, that it is in the range `0..=p-1`,
/// where `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`.
#[inline(always)]
#[must_use]
pub fn check_range_p(a: &[u32; 8]) -> bool {
    // SAFETY: `a` is valid for the duration of the call.
    unsafe { asm::P256_check_range_p(a) }
}

#[inline(always)]
pub(crate) fn negate_mod_n_if(out: &mut [u32; 8], inn: &[u32; 8], should_negate: bool) {
    // SAFETY: `out` and `inn` are valid for the duration of the call,
    // and `out` is valid for writes.
    unsafe { asm::P256_negate_mod_n_if(out, inn, should_negate as u32) };
}

#[inline(always)]
pub(crate) fn add_sub_j_affine(a: &mut [Montgomery; 3], b: &[Montgomery; 2], is_sub: bool) {
    // SAFETY: `a` and `b` are valid for the duration of the call, `a` is valid
    // for writes, and `b` has the correct length (2) for the value of `b_is_affine`.
    unsafe { asm::jacobian::P256_add_sub_j(a, b.as_ptr(), is_sub, true) }
}

#[inline(always)]
pub(crate) fn add_sub_j(a: &mut [Montgomery; 3], b: &[Montgomery; 3], is_sub: bool) {
    // SAFETY: `a` and `b` are valid for the duration of the call, `a` is valid
    // for writes, and `b` has the correct length (3) for the value of `b_is_affine`.
    unsafe { asm::jacobian::P256_add_sub_j(a, b.as_ptr(), is_sub, false) }
}

#[inline(always)]
pub(crate) fn double_j(jacobian_out: &mut [Montgomery; 3], jacobian_in: &[Montgomery; 3]) {
    // SAFETY: `jacobian_out` and `jacobian_in` are valid for the duration of the
    // function call, and `jacobian_out` is valid for writes.
    unsafe { asm::jacobian::P256_double_j(jacobian_out, jacobian_in) };
}

#[inline(always)]
pub(crate) fn double_j_inplace(jacobian: &mut [Montgomery; 3]) {
    // SAFETY: `jacobian` is valid for the duration of the functino call,
    // `jacobian` is valid for writes. Additionally, the read and write pointers
    // passed to `P256_double_j` may overlap.
    unsafe { asm::jacobian::P256_double_j(jacobian, jacobian) };
}

#[inline(always)]
pub fn jacobian_to_affine(x: &mut Montgomery, y: &mut Montgomery, jacobian: &[Montgomery; 3]) {
    // SAFETY: `x`, `y` and `jacobian` are valid for the duration of the function call,
    // `x` and `y` are valid for writes.
    unsafe { asm::jacobian::P256_jacobian_to_affine(x, y, jacobian) };
}
