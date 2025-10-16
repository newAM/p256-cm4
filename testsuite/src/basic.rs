#![no_std]
#![no_main]

use cortex_m::peripheral::DWT;
use defmt::unwrap;
use defmt_rtt as _; // global logger
use hex_literal::hex;
use panic_probe as _;

const FREQ: u32 = 48_000_000;
const CYC_PER_MICRO: u32 = FREQ / 1000 / 1000;

// WARNING will wrap-around eventually, use this for relative timing only
defmt::timestamp!("{=u32:us}", DWT::cycle_count() / CYC_PER_MICRO);

// Message hash
const HASH: [u32; 8] = [
    0xb7f6ac44, 0x42136ce3, 0x7289c5c2, 0x5009fe04, 0xfb2e1e4e, 0x7703901a, 0xa6e7c4db, 0x56ec33a1,
];

const PRIVATE_KEY: [u32; 8] = [
    0x3d429b51, 0x588b5f71, 0xeea84f1f, 0x1a77f459, 0x13c8445b, 0xac3e4e0b, 0x6da554ca, 0x64b472da,
];

// Note: in real-world use this should be a one-time random number (nonce).
// This fixed value is for testing purposes only.
const INTEGER: [u32; 8] = [
    0xb1bba194, 0x616a904b, 0x45f280a2, 0x7f3ce9f9, 0x47624a3b, 0x335d4f82, 0x870767b9, 0xde682a64,
];

const R_SIGN: [u32; 8] = [
    0x6180acf3, 0x5b7914b5, 0xd6e34388, 0xed279562, 0x1f6bfd2a, 0x7a5a556a, 0x6f5ebbca, 0xacc2c879,
];
const S_SIGN: [u32; 8] = [
    0x1978f78b, 0xb2a605ca, 0x26766c78, 0x1c37f72b, 0x18b297ef, 0x5a176fe9, 0x2adacd3c, 0x038905cc,
];

const CURVE_PT_X: [u32; 8] = [
    0x1ce9cb1c, 0xf4c75f07, 0xa2bf33f0, 0xcc8fdb48, 0xe95d56d3, 0x2fb1bf4b, 0x46ff593c, 0x83bf71c2,
];

const CURVE_PT_Y: [u32; 8] = [
    0xc61440ce, 0xa2f91188, 0x2cdb1f1a, 0xe013610e, 0x93cab76d, 0x784e40b7, 0x5ccd7cdc, 0xa94c9aa8,
];

fn into_bytes(i: [u32; 8]) -> [u8; 32] {
    unsafe { core::mem::transmute::<[u32; 8], [u8; 32]>(i) }
}

fn u32x8_to_u8x32(input: &[u32; 8]) -> &[u8; 32] {
    unsafe { core::mem::transmute::<&[u32; 8], &[u8; 32]>(input) }
}

fn u32x8_to_u8x32_mut(input: &mut [u32; 8]) -> &mut [u8; 32] {
    unsafe { core::mem::transmute::<&mut [u32; 8], &mut [u8; 32]>(input) }
}

#[defmt_test::tests]
mod tests {
    use p256_cm4::convert_endianness;

    use super::*;

    const ZERO: [u32; 8] = [0; 8];
    const ONE: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

    #[init]
    fn init() {
        let mut cp = unwrap!(cortex_m::peripheral::Peripherals::take());

        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();
        cp.DWT.set_cycle_count(0);
    }

    #[test]
    fn check_range_n() {
        use p256_cm4::check_range_n;

        let valid: bool = check_range_n(&ZERO);
        defmt::assert!(!valid, "0 is not in range");

        let valid: bool = check_range_n(&ONE);
        defmt::assert!(valid, "1 is in range");

        // 2**256 - 2**224 + 2**192 - 0x4319055258e8617b0c46353d039cdaaf
        const N: [u32; 8] = [
            0xfc632551, 0xf3b9cac2, 0xa7179e84, 0xbce6faad, 0xffffffff, 0xffffffff, 0x00000000,
            0xffffffff,
        ];
        let valid: bool = check_range_n(&N);
        defmt::assert!(!valid, "N is not within range");

        const N_MINUS_ONE: [u32; 8] = [
            0xfc632550, 0xf3b9cac2, 0xa7179e84, 0xbce6faad, 0xffffffff, 0xffffffff, 0x00000000,
            0xffffffff,
        ];
        let valid: bool = check_range_n(&N_MINUS_ONE);
        defmt::assert!(valid, "N - 1 is within range");
    }

    #[test]
    fn check_range_p() {
        use p256_cm4::check_range_p;

        let valid: bool = check_range_p(&ZERO);
        defmt::assert!(valid, "0 is in range");

        let valid: bool = check_range_p(&ONE);
        defmt::assert!(valid, "1 is in range");

        // 2**256 - 2**224 + 2**192 + 2**96 - 1
        const P: [u32; 8] = [
            0xffffffff, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000001,
            0xffffffff,
        ];
        let valid: bool = check_range_p(&P);
        defmt::assert!(!valid, "P is not within range");

        const P_MINUS_ONE: [u32; 8] = [
            0xfffffffe, 0xffffffff, 0xffffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000001,
            0xffffffff,
        ];
        let valid: bool = check_range_p(&P_MINUS_ONE);
        defmt::assert!(valid, "P - 1 is within range");
    }

    #[test]
    fn test_convert_endianness() {
        use p256_cm4::convert_endianness;

        const INPUT: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let mut output: [u32; 8] = [0; 8];
        convert_endianness(u32x8_to_u8x32_mut(&mut output), &INPUT);
        defmt::assert_eq!(
            output,
            [
                0x1C1D1E1F, 0x18191A1B, 0x14151617, 0x10111213, 0x0C0D0E0F, 0x08090A0B, 0x04050607,
                0x00010203
            ]
        );
    }

    const X: [u32; 8] = [
        0x00112233, 0x44556677, 0x8899AABB, 0xCCDDEEFF, 0x00112233, 0x44556677, 0x8899AABB,
        0xCCDDEEFF,
    ];
    const Y: [u32; 8] = [
        0x01234567, 0x89ABCDEF, 0x12345678, 0x9ABCDEF0, 0x01234567, 0x89ABCDEF, 0x12345678,
        0x9ABCDEF0,
    ];

    #[test]
    fn test_point_to_octet_string_uncompressed() {
        use p256_cm4::point_to_octet_string_uncompressed;

        let mut out: [u8; 65] = [0; 65];
        point_to_octet_string_uncompressed(&mut out, &X, &Y);
        defmt::assert_eq!(
            out,
            [
                0x04, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77, 0x00,
                0x11, 0x22, 0x33, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66,
                0x77, 0x00, 0x11, 0x22, 0x33, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x89,
                0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
                0x78, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67
            ]
        );
    }

    #[test]
    fn test_point_to_octet_string_compressed() {
        use p256_cm4::point_to_octet_string_compressed;

        let mut out: [u8; 33] = [0; 33];
        point_to_octet_string_compressed(&mut out, &X, &Y);
        defmt::assert_eq!(
            out,
            [
                0x03, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77, 0x00,
                0x11, 0x22, 0x33, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66,
                0x77, 0x00, 0x11, 0x22, 0x33,
            ]
        );
    }

    #[test]
    fn test_point_to_octet_string_hybrid() {
        use p256_cm4::point_to_octet_string_hybrid;

        let mut out: [u8; 65] = [0; 65];
        point_to_octet_string_hybrid(&mut out, &X, &Y);
        defmt::assert_eq!(
            out,
            [
                0x07, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77, 0x00,
                0x11, 0x22, 0x33, 0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66,
                0x77, 0x00, 0x11, 0x22, 0x33, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x89,
                0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56,
                0x78, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67
            ]
        );
    }

    #[test]
    fn test_octet_string_to_point() {
        use p256_cm4::octet_string_to_point;

        const DER: [u8; 65] = [
            0x04, 0x57, 0x63, 0x64, 0xFF, 0xC3, 0x07, 0xBC, 0x8E, 0x7C, 0x2A, 0xB0, 0xB4, 0x91,
            0x0B, 0xB6, 0x70, 0xAE, 0x47, 0x29, 0x62, 0xFC, 0x7B, 0xE6, 0x41, 0x41, 0xA1, 0xF5,
            0x65, 0x5F, 0x2C, 0xC8, 0x56, 0xAB, 0xB2, 0xB2, 0x25, 0x73, 0x5F, 0x32, 0x77, 0x5B,
            0xDD, 0x82, 0x45, 0x98, 0x96, 0xFD, 0x3A, 0x92, 0x8C, 0x04, 0x0F, 0xB1, 0x33, 0x87,
            0x8E, 0xE9, 0xAC, 0x79, 0xE1, 0x72, 0x9E, 0x92, 0xE3,
        ];

        let mut x: [u32; 8] = [0; 8];
        let mut y: [u32; 8] = [0; 8];

        let is_ok: bool = octet_string_to_point(&mut x, &mut y, &DER);
        defmt::assert!(is_ok, "An error occured");
        defmt::assert_eq!(
            x,
            [
                0x5F2CC856, 0x41A1F565, 0xFC7BE641, 0xAE472962, 0x910BB670, 0x7C2AB0B4, 0xC307BC8E,
                0x576364FF
            ]
        );
        defmt::assert_eq!(
            y,
            [
                0x729E92E3, 0xE9AC79E1, 0xB133878E, 0x928C040F, 0x9896FD3A, 0x5BDD8245, 0x735F3277,
                0xABB2B225
            ]
        );
    }

    #[test]
    fn test_verify() {
        use p256_cm4::{convert_endianness, octet_string_to_point, verify};

        let start: u32 = DWT::cycle_count();
        let mut key: [u8; 65] = [0; 65];
        key[0] = 0x04;
        key[1..33].copy_from_slice(&into_bytes(CURVE_PT_X));
        key[33..65].copy_from_slice(&into_bytes(CURVE_PT_Y));

        let mut x: [u32; 8] = [0; 8];
        let mut y: [u32; 8] = [0; 8];

        let is_ok: bool = octet_string_to_point(&mut x, &mut y, &key);
        assert!(is_ok, "p256_octet_string_to_point");

        let mut r: [u32; 8] = [0; 8];
        let mut s: [u32; 8] = [0; 8];

        convert_endianness(u32x8_to_u8x32_mut(&mut r), u32x8_to_u8x32(&R_SIGN));
        convert_endianness(u32x8_to_u8x32_mut(&mut s), u32x8_to_u8x32(&S_SIGN));

        let authentic: bool = verify(
            &x,
            &y,
            unsafe { core::mem::transmute::<&[u32; 8], &[u8; 32]>(&HASH) },
            &r,
            &s,
        );

        let elapsed: u32 = DWT::cycle_count().wrapping_sub(start);

        defmt::info!("Approximate cycles per p256 verify: {}", elapsed);

        defmt::assert!(authentic);
    }

    #[test]
    fn sec1_compressed_even_parity() {
        use p256_cm4::octet_string_to_point;

        // The expected X coordinate (big-endian)
        let x = [
            0x6F, 0xF0, 0x3B, 0x94, 0x92, 0x41, 0xCE, 0x1D, 0xAD, 0xD4, 0x35, 0x19, 0xE6, 0x96,
            0x0E, 0x0A, 0x85, 0xB4, 0x1A, 0x69, 0xA0, 0x5C, 0x32, 0x81, 0x03, 0xAA, 0x2B, 0xCE,
            0x15, 0x94, 0xCA, 0x16,
        ];

        // The expected Y coordinate (big-endian)
        let y = [
            0x3C, 0x4F, 0x75, 0x3A, 0x55, 0xBF, 0x01, 0xDC, 0x53, 0xF6, 0xC0, 0xB0, 0xC7, 0xEE,
            0xE7, 0x8B, 0x40, 0xC6, 0xFF, 0x7D, 0x25, 0xA9, 0x6E, 0x22, 0x82, 0xB9, 0x89, 0xCE,
            0xF7, 0x1C, 0x14, 0x4A,
        ];

        // sec1 compressed data, even parity
        let sec1_compressed = [
            0x02, 0x6F, 0xF0, 0x3B, 0x94, 0x92, 0x41, 0xCE, 0x1D, 0xAD, 0xD4, 0x35, 0x19, 0xE6,
            0x96, 0x0E, 0x0A, 0x85, 0xB4, 0x1A, 0x69, 0xA0, 0x5C, 0x32, 0x81, 0x03, 0xAA, 0x2B,
            0xCE, 0x15, 0x94, 0xCA, 0x16,
        ];

        let mut x_expected = [0u32; 8];
        let mut y_expected = [0u32; 8];
        convert_endianness(u32x8_to_u8x32_mut(&mut x_expected), &x);
        convert_endianness(u32x8_to_u8x32_mut(&mut y_expected), &y);

        let mut x_out = [0u32; 8];
        let mut y_out = [0u32; 8];

        defmt::assert!(octet_string_to_point(
            &mut x_out,
            &mut y_out,
            &sec1_compressed
        ));

        defmt::assert_eq!(x_expected, x_out);
        defmt::assert_eq!(y_expected, y_out);
    }

    #[test]
    fn sec1_compressed_odd_parity() {
        use p256_cm4::octet_string_to_point;

        // The expected X coordinate (big-endian)
        let x = [
            0x6F, 0xF0, 0x3B, 0x94, 0x92, 0x41, 0xCE, 0x1D, 0xAD, 0xD4, 0x35, 0x19, 0xE6, 0x96,
            0x0E, 0x0A, 0x85, 0xB4, 0x1A, 0x69, 0xA0, 0x5C, 0x32, 0x81, 0x03, 0xAA, 0x2B, 0xCE,
            0x15, 0x94, 0xCA, 0x16,
        ];

        // The expected Y coordinate (big-endian)
        let y = [
            0xC3, 0xB0, 0x8A, 0xC4, 0xAA, 0x40, 0xFE, 0x24, 0xAC, 0x09, 0x3F, 0x4F, 0x38, 0x11,
            0x18, 0x74, 0xBF, 0x39, 0x00, 0x83, 0xDA, 0x56, 0x91, 0xDD, 0x7D, 0x46, 0x76, 0x31,
            0x08, 0xE3, 0xEB, 0xB5,
        ];

        // sec1 compressed data, odd parity
        let sec1_compressed = [
            0x03, 0x6F, 0xF0, 0x3B, 0x94, 0x92, 0x41, 0xCE, 0x1D, 0xAD, 0xD4, 0x35, 0x19, 0xE6,
            0x96, 0x0E, 0x0A, 0x85, 0xB4, 0x1A, 0x69, 0xA0, 0x5C, 0x32, 0x81, 0x03, 0xAA, 0x2B,
            0xCE, 0x15, 0x94, 0xCA, 0x16,
        ];

        let mut x_expected = [0u32; 8];
        let mut y_expected = [0u32; 8];

        convert_endianness(u32x8_to_u8x32_mut(&mut x_expected), &x);
        convert_endianness(u32x8_to_u8x32_mut(&mut y_expected), &y);

        let mut x_out = [0u32; 8];
        let mut y_out = [0u32; 8];

        defmt::assert!(octet_string_to_point(
            &mut x_out,
            &mut y_out,
            &sec1_compressed
        ));

        defmt::assert_eq!(x_expected, x_out);
        defmt::assert_eq!(y_expected, y_out);
    }

    #[test]
    fn test_sign() {
        use p256_cm4::{check_range_n, convert_endianness, sign};

        let start: u32 = DWT::cycle_count();
        let mut private_key: [u32; 8] = [0; 8];

        convert_endianness(
            u32x8_to_u8x32_mut(&mut private_key),
            &into_bytes(PRIVATE_KEY),
        );

        defmt::assert!(check_range_n(&private_key));

        let mut integer: [u32; 8] = [0; 8];

        convert_endianness(u32x8_to_u8x32_mut(&mut integer), &into_bytes(INTEGER));

        let mut r_sign: [u32; 8] = [0; 8];
        let mut s_sign: [u32; 8] = [0; 8];

        let is_ok: bool = sign(
            &mut r_sign,
            &mut s_sign,
            unsafe { core::mem::transmute::<&[u32; 8], &[u8; 32]>(&HASH) },
            &private_key,
            &integer,
        );
        let elapsed: u32 = DWT::cycle_count().wrapping_sub(start);

        defmt::info!("Approximate cycles per p256 sign: {}", elapsed);

        let mut r: [u32; 8] = [0; 8];
        let mut s: [u32; 8] = [0; 8];

        convert_endianness(u32x8_to_u8x32_mut(&mut r), u32x8_to_u8x32(&r_sign));
        convert_endianness(u32x8_to_u8x32_mut(&mut s), u32x8_to_u8x32(&s_sign));

        defmt::assert!(is_ok, "An error occured");
        defmt::debug!("r={:08X}", r);
        defmt::debug!("R_SIGN={:08X}", R_SIGN);
        defmt::debug!("s={:08X}", s);
        defmt::debug!("S_SIGN={:08X}", S_SIGN);
        defmt::assert_eq!(r, R_SIGN);
        defmt::assert_eq!(s, S_SIGN);
    }

    // TODO: clean up this test, these values are hard-coded from something that I know works
    #[test]
    fn test_ecdh() {
        use p256_cm4::{
            check_range_n, convert_endianness, ecdh_calc_shared_secret, octet_string_to_point,
        };

        let mut shared_secret: [u8; 32] = [0; 32];
        let public_key_bytes: [u8; 65] = hex!(
            "04ae981c0a88d381a88e3e9999d9feb0e068c918b9b4ff5e015f8d1be714c73cf61145b96af854c98bdd737d7b85fbce82a2e4f613ee82f4864e9bd906808c26d9"
        );

        let mut public_x: [u32; 8] = [0; 8];
        let mut public_y: [u32; 8] = [0; 8];

        const PRIV_KEY_BYTES: [u8; 32] = [
            110, 14, 138, 238, 3, 36, 199, 102, 123, 228, 243, 149, 14, 155, 38, 126, 30, 98, 62,
            79, 177, 166, 27, 110, 153, 72, 248, 124, 20, 10, 210, 96,
        ];

        let mut key: [u32; 8] = [0; 8];

        convert_endianness(u32x8_to_u8x32_mut(&mut key), &PRIV_KEY_BYTES);

        assert!(check_range_n(&key));
        defmt::assert_eq!(
            key,
            [
                336253536, 2571696252, 2980453230, 509754959, 245048958, 2078602133, 52741990,
                1846446830
            ]
        );

        let is_ok: bool = octet_string_to_point(&mut public_x, &mut public_y, &public_key_bytes);
        defmt::assert!(is_ok);

        defmt::assert_eq!(
            public_x,
            [
                348601590, 1603083239, 3036634625, 1758009529, 3657347296, 2386467225, 2295562664,
                2929204234
            ]
        );
        defmt::assert_eq!(
            public_y,
            [
                2156668633, 1318836486, 4001559686, 2732914195, 2247872130, 3715333499, 4166306187,
                289782122
            ]
        );

        let point_is_on_curve: bool =
            ecdh_calc_shared_secret(&mut shared_secret, &key, &public_x, &public_y);
        defmt::assert!(point_is_on_curve);

        defmt::assert_eq!(
            shared_secret,
            [
                130, 67, 76, 181, 106, 118, 45, 28, 18, 174, 221, 26, 193, 186, 97, 133, 156, 81,
                36, 219, 191, 249, 107, 208, 133, 19, 221, 61, 9, 186, 157, 167
            ]
        );
    }
}
