use crate::{Montgomery, check_range_n, check_range_p, sys, verify_no_bounds_checks};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub enum VerifyingKeyError {
    /// Either the `x` or `y` coordinate of the provided point
    /// were outside of the range `0..=p - 1`.
    OutOfRange,
    /// The provided point was not on the `p256` curve.
    NotOnCurve,
}

/// A verifying key, also called a public key.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub struct VerifyingKey {
    x: [u32; 8],
    y: [u32; 8],
}

impl VerifyingKey {
    /// Create a new [`VerifyingKey`] from the provided raw, big-endian encoded
    /// `x` and `y` coordinates.
    ///
    /// An error is returned if `x` or `y` is not in the range `0..=p - 1`, where `p`
    /// is the `p256` prime, or if the point `(x, y)` is not on the `p256` curve.
    pub fn from_parts(x: &[u8; 32], y: &[u8; 32]) -> Result<VerifyingKey, VerifyingKeyError> {
        let x = to_little_endian(x);
        let y = to_little_endian(y);

        if !check_range_p(&x) || !check_range_p(&y) {
            return Err(VerifyingKeyError::OutOfRange);
        }

        let x_mont = Montgomery::from(x);
        let y_mont = Montgomery::from(y);

        sys::point_is_on_curve(&x_mont, &y_mont)
            .then_some(Self { x, y })
            .ok_or(VerifyingKeyError::NotOnCurve)
    }

    /// Verify that the private-key counterpart to this [`VerifyingKey`] has produced
    /// `signature` by signing `hash`.
    pub fn verify_prehash(&self, hash: &[u8; 32], signature: &Signature) -> bool {
        verify_no_bounds_checks(&self.x, &self.y, hash, &signature.r, &signature.s)
    }
}

/// A signature.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    r: [u32; 8],
    s: [u32; 8],
}

impl Signature {
    /// Create a new [`Signature`] from the provided bytes.
    ///
    /// The first 32 bytes of the `bytes` are interpreted as the `r`
    /// value of this [`Signature`], and the second 32 bytes are interpreted
    /// as the `s` value. An error is returned if the two values don't
    /// constitute a valid [`Signature`] (see [`Signature::from_parts`] for
    /// more information on validity).
    pub fn from_bytes(bytes: &[u8; 64]) -> Option<Self> {
        Self::from_parts(
            bytes[..32].try_into().unwrap(),
            bytes[32..].try_into().unwrap(),
        )
    }

    /// Create a new [`Signature`] from the little-endian encoded values
    /// `r` and `s`.
    ///
    /// An error is returned if `r` or `s` is not in the range `1..=n - 1`, where
    /// `n` is the `p256` order.
    pub fn from_parts(r: &[u8; 32], s: &[u8; 32]) -> Option<Self> {
        let r = to_little_endian(r);
        let s = to_little_endian(s);

        if !check_range_n(&r) || !check_range_n(&s) {
            return None;
        }

        Some(Self { r, s })
    }
}

/// Convert an input 256 bit number (big-endian u8s) into the format
/// used by this library (little-endian u32)
fn to_little_endian(input: &[u8; 32]) -> [u32; 8] {
    let mut output = [0u32; 8];

    for (idx, chunk) in input.chunks_exact(4).enumerate() {
        output[output.len() - 1 - idx] = u32::from_be_bytes(chunk.try_into().unwrap());
    }

    output
}
