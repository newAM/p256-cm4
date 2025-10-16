#![no_std]
#![no_main]
#![cfg(test)]

use cortex_m::peripheral::DWT;

#[cfg(not(feature = "rtt"))]
use defmt_semihosting as _;

#[cfg(feature = "rtt")]
use defmt_rtt as _;

const FREQ: u32 = 48_000_000;
const CYC_PER_MICRO: u32 = FREQ / 1000 / 1000;

// WARNING will wrap-around eventually, use this for relative timing only
defmt::timestamp!("{=u32:us}", DWT::cycle_count() / CYC_PER_MICRO);

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use cortex_m_semihosting::debug;

    defmt::error!("{}", defmt::Display2Format(info));
    debug::exit(debug::EXIT_FAILURE);
    loop {}
}

// Message hash
const HASH: [u8; 32] = *u32x8_to_u8x32(&[
    0xb7f6ac44, 0x42136ce3, 0x7289c5c2, 0x5009fe04, 0xfb2e1e4e, 0x7703901a, 0xa6e7c4db, 0x56ec33a1,
]);

const R_SIGN: [u8; 32] = *u32x8_to_u8x32(&[
    0x6180acf3, 0x5b7914b5, 0xd6e34388, 0xed279562, 0x1f6bfd2a, 0x7a5a556a, 0x6f5ebbca, 0xacc2c879,
]);
const S_SIGN: [u8; 32] = *u32x8_to_u8x32(&[
    0x1978f78b, 0xb2a605ca, 0x26766c78, 0x1c37f72b, 0x18b297ef, 0x5a176fe9, 0x2adacd3c, 0x038905cc,
]);

const CURVE_PT_X: [u8; 32] = *u32x8_to_u8x32(&[
    0x1ce9cb1c, 0xf4c75f07, 0xa2bf33f0, 0xcc8fdb48, 0xe95d56d3, 0x2fb1bf4b, 0x46ff593c, 0x83bf71c2,
]);

const CURVE_PT_Y: [u8; 32] = *u32x8_to_u8x32(&[
    0xc61440ce, 0xa2f91188, 0x2cdb1f1a, 0xe013610e, 0x93cab76d, 0x784e40b7, 0x5ccd7cdc, 0xa94c9aa8,
]);

const fn u32x8_to_u8x32(input: &[u32; 8]) -> &[u8; 32] {
    unsafe { core::mem::transmute::<&[u32; 8], &[u8; 32]>(input) }
}

#[defmt_test::tests]
mod tests {
    use super::*;
    use p256_cm4::{Signature, VerifyingKey, VerifyingKeyError};

    #[init]
    fn init() {
        let mut cp = defmt::unwrap!(cortex_m::peripheral::Peripherals::take());

        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();
        cp.DWT.set_cycle_count(0);
    }

    #[test]
    fn verifying_key_x_too_large() {
        let large_x = [u8::MAX; 32];
        defmt::assert_eq!(
            VerifyingKey::from_parts(&large_x, &CURVE_PT_Y),
            Err(VerifyingKeyError::OutOfRange)
        );
    }

    #[test]
    fn verifying_key_y_too_large() {
        let large_y = [u8::MAX; 32];
        defmt::assert_eq!(
            VerifyingKey::from_parts(&CURVE_PT_X, &large_y),
            Err(VerifyingKeyError::OutOfRange)
        );
    }

    #[test]
    fn verifying_key_not_on_curve_y() {
        let mut y_plus_one = CURVE_PT_Y.clone();
        y_plus_one[0] += 1;
        defmt::assert_eq!(
            VerifyingKey::from_parts(&CURVE_PT_X, &y_plus_one),
            Err(VerifyingKeyError::NotOnCurve)
        );
    }

    #[test]
    fn verifying_key_not_on_curve_x() {
        let mut x_plus_one = CURVE_PT_X.clone();
        x_plus_one[0] += 1;
        defmt::assert_eq!(
            VerifyingKey::from_parts(&x_plus_one, &CURVE_PT_Y),
            Err(VerifyingKeyError::NotOnCurve)
        );
    }

    #[test]
    fn signature_r_too_small() {
        let small_r = [u8::MIN; 32];
        defmt::assert!(Signature::from_parts(&small_r, &R_SIGN).is_none());
    }

    #[test]
    fn signature_r_too_large() {
        let large_r = [u8::MAX; 32];
        defmt::assert!(Signature::from_parts(&large_r, &R_SIGN).is_none());
    }

    #[test]
    fn signature_s_too_small() {
        let small_s = [u8::MIN; 32];
        defmt::assert!(Signature::from_parts(&S_SIGN, &small_s).is_none());
    }

    #[test]
    fn signature_s_too_large() {
        let large_s = [u8::MAX; 32];
        defmt::assert!(Signature::from_parts(&S_SIGN, &large_s).is_none());
    }

    #[test]
    fn verify() {
        let start: u32 = DWT::cycle_count();

        let key = VerifyingKey::from_parts(&CURVE_PT_X, &CURVE_PT_Y).unwrap();
        let signature = Signature::from_parts(&R_SIGN, &S_SIGN).unwrap();

        let authentic = key.verify_prehash(&HASH, &signature);

        let elapsed: u32 = DWT::cycle_count().wrapping_sub(start);

        defmt::info!("Approximate cycles per p256 verify: {}", elapsed);

        defmt::assert!(authentic);
    }

    #[test]
    fn dont_verify_incorrect() {
        let key = VerifyingKey::from_parts(&CURVE_PT_X, &CURVE_PT_Y).unwrap();
        let signature = Signature::from_parts(&R_SIGN, &S_SIGN).unwrap();

        let mut hash = HASH.clone();
        hash[0] += 1;
        let authentic = key.verify_prehash(&hash, &signature);
        defmt::assert!(!authentic);
    }

    #[test]
    fn sec1_roundtrip() {
        let mut key: [u8; 65] = [0; 65];
        key[0] = 0x04;
        key[1..33].copy_from_slice(&CURVE_PT_X);
        key[33..65].copy_from_slice(&CURVE_PT_Y);

        let verifying_key = VerifyingKey::from_sec1_bytes(&key).unwrap();

        let encoded = verifying_key.to_sec1_bytes();

        defmt::assert_eq!(key, encoded);
    }

    #[test]
    fn sec1_compressed_roundtrip_odd_parity() {
        let mut key = [0; 33];
        key[0] = 0x03;
        key[1..33].copy_from_slice(&CURVE_PT_X);

        let verifying_key = VerifyingKey::from_sec1_bytes(&key).unwrap();
        let (_, y) = verifying_key.to_bytes();

        let encoded = verifying_key.to_sec1_bytes_compressed();

        defmt::assert_eq!(key, encoded);
        defmt::assert_eq!(&y, &CURVE_PT_Y);
    }

    #[test]
    fn sec1_compressed_roundtrip_even_parity() {
        let mut key = [0; 33];
        key[0] = 0x02;
        key[1..33].copy_from_slice(&CURVE_PT_X);

        let verifying_key = VerifyingKey::from_sec1_bytes(&key).unwrap();
        let (_, y) = verifying_key.to_bytes();

        // Expected y coordinate when decoding `x` as compressed point with
        // odd parity.
        let expected_y = [
            0x31, 0xBF, 0xEB, 0x38, 0x77, 0xEE, 0x06, 0x5E, 0xE5, 0xE0, 0x24, 0xD3, 0xF1, 0x9E,
            0xEC, 0x1F, 0x92, 0x48, 0x35, 0x6D, 0x48, 0xBF, 0xB1, 0x87, 0x23, 0x83, 0x32, 0xA3,
            0x57, 0x65, 0xB3, 0x56,
        ];

        let encoded = verifying_key.to_sec1_bytes_compressed();

        defmt::assert_eq!(key, encoded);
        defmt::assert_eq!(y, expected_y)
    }

    #[test]
    fn sec1_hybrid_odd() {
        let mut key = [0; 65];
        key[0] = 0x07;
        key[1..33].copy_from_slice(&CURVE_PT_X);
        key[33..65].copy_from_slice(&CURVE_PT_Y);

        let verifying_key = VerifyingKey::from_sec1_bytes(&key).unwrap();
        let (_, y) = verifying_key.to_bytes();

        defmt::assert_eq!(&y, &CURVE_PT_Y);
    }

    #[test]
    fn sec1_hybrid_even() {
        // Expected y coordinate at `x` with odd parity.
        let expected_y = [
            0x31, 0xBF, 0xEB, 0x38, 0x77, 0xEE, 0x06, 0x5E, 0xE5, 0xE0, 0x24, 0xD3, 0xF1, 0x9E,
            0xEC, 0x1F, 0x92, 0x48, 0x35, 0x6D, 0x48, 0xBF, 0xB1, 0x87, 0x23, 0x83, 0x32, 0xA3,
            0x57, 0x65, 0xB3, 0x56,
        ];

        let mut key = [0; 65];
        key[0] = 0x06;
        key[1..33].copy_from_slice(&CURVE_PT_X);
        key[33..65].copy_from_slice(&expected_y);

        let verifying_key = VerifyingKey::from_sec1_bytes(&key).unwrap();
        let (_, y) = verifying_key.to_bytes();

        defmt::assert_eq!(&y, &expected_y);
    }

    #[test]
    fn sec1_empty() {
        defmt::assert_eq!(
            VerifyingKey::from_sec1_bytes(&[]),
            Err(p256_cm4::VerifyingKeySec1Error::TooLittleData)
        );
    }

    #[test]
    fn sec1_uncompressed_too_little_data_x() {
        let data = [0x04; 32];
        defmt::assert_eq!(
            VerifyingKey::from_sec1_bytes(&data),
            Err(p256_cm4::VerifyingKeySec1Error::TooLittleData)
        );
    }
    #[test]
    fn sec1_uncompressed_too_little_data_y() {
        let data = [0x04; 64];
        defmt::assert_eq!(
            VerifyingKey::from_sec1_bytes(&data),
            Err(p256_cm4::VerifyingKeySec1Error::TooLittleData)
        );
    }

    #[test]
    fn sec1_too_little_data_x() {
        let data = [0x06; 32];
        defmt::assert_eq!(
            VerifyingKey::from_sec1_bytes(&data),
            Err(p256_cm4::VerifyingKeySec1Error::TooLittleData)
        );
    }

    #[test]
    fn sec1_too_little_data_y() {
        let data = [0x06; 33];
        defmt::assert_eq!(
            VerifyingKey::from_sec1_bytes(&data),
            Err(p256_cm4::VerifyingKeySec1Error::TooLittleData)
        );
    }
}
