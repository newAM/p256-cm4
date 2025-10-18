#[path = "./asm/mod.rs"]
#[cfg(target_arch = "arm")]
pub(crate) mod asm;

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
