use crate::{
    Montgomery, SignPrecomp, check_range_n, check_range_p, sign_step1, sign_step2, sys,
    verify_no_bounds_checks,
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub enum VerifyingKeyError {
    /// Either the `x` or `y` coordinate of the provided point
    /// were outside of the range `0..=p - 1`.
    OutOfRange,
    /// The provided point was not on the `p256` curve.
    NotOnCurve,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub enum VerifyingKeySec1Error {
    /// An error occurred with the key that was decoded.
    Key(VerifyingKeyError),
    /// The provided amount of data was too small and could
    /// not be decoded.
    TooLittleData,
    /// The provided data had an invalid `sec1` tag.
    InvalidTag,
    /// The parity data did not match.
    InvalidParity,
}

/// A verifying key, also called a public key.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub struct VerifyingKey {
    pub(crate) x: [u32; 8],
    pub(crate) y: [u32; 8],
}

impl VerifyingKey {
    fn from_parts_valid_range(x: [u32; 8], y: [u32; 8]) -> Result<Self, VerifyingKeyError> {
        let x_mont = Montgomery::from(x);
        let y_mont = Montgomery::from(y);

        sys::point_is_on_curve(&x_mont, &y_mont)
            .then_some(Self { x, y })
            .ok_or(VerifyingKeyError::NotOnCurve)
    }

    /// Create a new [`VerifyingKey`] from the provided raw, big-endian encoded
    /// `x` and `y` coordinates.
    ///
    /// An error is returned if `x` or `y` is not in the range `0..=p - 1`, where `p`
    /// is the `p256` prime, or if the point `(x, y)` is not on the `p256` curve.
    pub fn from_parts(x: &[u8; 32], y: &[u8; 32]) -> Result<Self, VerifyingKeyError> {
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

    /// Convert this [`VerifyingKey`] to its [`sec1`] representation (uncompressed).
    ///
    /// [`sec1`]: https://docs.rs/sec1/latest/sec1/
    pub fn to_sec1_bytes(&self) -> [u8; 65] {
        let mut output = [0u8; 65];
        output[0] = 4;
        to_big_endian(&self.x, (&mut output[1..33]).try_into().unwrap());
        to_big_endian(&self.y, (&mut output[33..65]).try_into().unwrap());
        output
    }

    /// Convert this [`VerifyingKey`] to its [`sec1`] representation (compressed).
    ///
    /// [`sec1`]: https://docs.rs/sec1/latest/sec1/
    pub fn to_sec1_bytes_compressed(&self) -> [u8; 33] {
        let mut output = [0u8; 33];
        to_big_endian(&self.x, (&mut output[1..33]).try_into().unwrap());
        output[0] = if self.y[0] & 0x1 == 0x1 { 3 } else { 2 };
        output
    }

    /// Parse the provided `data` as [`sec1`] bytes and return the
    /// result if it constitutes a valid [`VerifyingKey`] (see
    /// [`VerifyingKey::from_parts`] for more information on validity).
    ///
    /// This implementation supports the uncompressed (`0x04`), hybrid (`0x06`, `0x07`),
    /// and compressed (`0x03`, `0x04`) forms. The compact form (`0x05`) is not supported.
    ///
    /// [`sec1`]: https://docs.rs/sec1/latest/sec1/
    pub fn from_sec1_bytes(data: &[u8]) -> Result<Self, VerifyingKeySec1Error> {
        let (tag, xy) = data
            .split_at_checked(1)
            .ok_or(VerifyingKeySec1Error::TooLittleData)?;

        let tag = tag[0];

        let (x, y) = xy
            .split_at_checked(32)
            .ok_or(VerifyingKeySec1Error::TooLittleData)?;

        let x: &[u8; 32] = x.try_into().unwrap();

        if tag == 4 || tag == 6 || tag == 7 {
            let y: &[u8; 32] = y
                .try_into()
                .map_err(|_| VerifyingKeySec1Error::TooLittleData)?;

            if tag == 6 || tag == 7 {
                let expected_parity = tag & 0x1;
                let parity = y[31] & 0x1;

                if expected_parity != parity {
                    return Err(VerifyingKeySec1Error::InvalidParity);
                }
            }

            Self::from_parts(x, y).map_err(VerifyingKeySec1Error::Key)
        } else if tag == 2 || tag == 3 {
            let x = to_little_endian(x);
            let mut y = [0u32; 8];

            let valid = sys::decompress_point(&mut y, &x, (u32::from(tag) & 0x1) == 1);

            if valid {
                Self::from_parts_valid_range(x, y).map_err(VerifyingKeySec1Error::Key)
            } else {
                Err(VerifyingKeySec1Error::Key(VerifyingKeyError::OutOfRange))
            }
        } else {
            Err(VerifyingKeySec1Error::InvalidTag)
        }
    }

    /// Convert this [`VerifyingKey`] into a tuple of little-endian integers
    /// describing the `x` and `y` coordinates.
    pub fn to_bytes(&self) -> ([u8; 32], [u8; 32]) {
        let mut x = [0u8; 32];
        let mut y = [0u8; 32];

        to_big_endian(&self.x, &mut x);
        to_big_endian(&self.y, &mut y);

        (x, y)
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

/// A signing key, also called a private key.
pub struct SigningKey {
    key_data: [u32; 8],
}

impl SigningKey {
    /// Create a new signing key from the provided key data.
    ///
    /// This function will return `None` if `key` is not in the range `1..=n-1` (where `n` is the
    /// `p256` order) when interpreted as a big-endian integer.
    pub fn new(key: &[u8; 32]) -> Option<Self> {
        let key_data = to_little_endian(key);

        check_range_n(&key_data).then_some(Self { key_data })
    }

    /// Sign the provided cryptographic hash `hash` using the securely randomly generated
    /// nonce `k`.
    ///
    /// This function returns `None` for some combinations of `self` and `k`.
    /// When it does, you must generate a new, securely randomly generated nonce
    /// `k` and call this function again.
    pub fn sign_with_k(&self, hash: &[u8; 32], k: &[u8; 32]) -> Option<Signature> {
        let mut r = Default::default();
        let mut s = Default::default();
        let mut precomp = SignPrecomp::default();
        let k = to_little_endian(k);

        (sign_step1(&mut precomp, &k)
            && sign_step2(&mut r, &mut s, hash, &self.key_data, &mut precomp))
        .then_some(Signature { r, s })
    }

    /// Sign the provided cryptographic hash `hash`.
    ///
    /// This function uses the implementation defined in `RFC6979` to
    /// produce a valid nonce for the signature, and is therefore somewhat
    /// slower than [`Self::sign_with`].
    #[cfg(feature = "rfc6979")]
    pub fn sign(&self, hash: &[u8; 32]) -> Signature {
        use sha2::Sha256;

        let mut le_key_data = Default::default();
        to_big_endian(&self.key_data, &mut le_key_data);

        let mut hmac = rfc6979::HmacDrbg::<Sha256>::new(&le_key_data, hash, &[]);
        let mut k = [0u8; 32];
        hmac.fill_bytes(&mut k);

        loop {
            if let Some(signature) = self.sign_with_k(hash, &k) {
                return signature;
            }
            hmac.fill_bytes(&mut k);
        }
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

/// Convert an input 256 bit number (little-endian u32s) into the format
/// used by `sec1` (big-endian u8s)
fn to_big_endian(input: &[u32; 8], output: &mut [u8; 32]) {
    for (idx, value) in input.iter().enumerate() {
        let start = output.len() - (idx + 1) * 4;
        let end = output.len() - (idx * 4);
        output[start..end].copy_from_slice(&value.to_be_bytes());
    }
}
