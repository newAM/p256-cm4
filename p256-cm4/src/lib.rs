#![no_std]
#![allow(clippy::missing_safety_doc)]

use crate::asm::{
    P256_decompress_point, P256_double_j, P256_from_montgomery, P256_point_is_on_curve,
    P256_to_montgomery, P256_verify_last_step, jacobian::P256_add_sub_j,
};

#[cfg(target_arch = "arm")]
core::arch::global_asm!(include_str!("./asm.s"), options(raw));

#[cfg(target_arch = "arm")]
mod asm;

unsafe extern "C" {

    // int P256_divsteps2_31(int delta, uint32_t f, uint32_t g, uint32_t res_matrix[4]);
    fn P256_divsteps2_31(delta: i32, f: u32, g: u32, res_matrix: *mut u32) -> i32;
    // void P256_matrix_mul_fg_9(uint32_t a, uint32_t b, const struct FGInteger fg[2], struct FGInteger *res);
    fn P256_matrix_mul_fg_9(a: u32, b: u32, fg: *const FGInteger, res: *mut FGInteger);

    // void P256_mul_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    fn P256_mul_mod_n(res: *mut u32, a: *const u32, b: *const u32);
    // void P256_add_mod_n(uint32_t res[8], const uint32_t a[8], const uint32_t b[8]);
    fn P256_add_mod_n(res: *mut u32, a: *const u32, b: *const u32);
    // void P256_reduce_mod_n_32bytes(uint32_t res[8], const uint32_t a[8]);
    fn P256_reduce_mod_n_32bytes(res: *mut u32, a: *const u32);

    // void P256_jacobian_to_affine(uint32_t affine_mont_x[8], uint32_t affine_mont_y[8], const uint32_t jacobian_mont[3][8]);
    fn P256_jacobian_to_affine(
        affine_mont_x: *mut u32,
        affine_mont_y: *mut u32,
        jacobian_mont: *const [u32; 8],
    );

    // void P256_negate_mod_p_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    fn P256_negate_mod_p_if(out: *mut u32, inn: *const u32, should_negate: u32);
    // void P256_negate_mod_n_if(uint32_t out[8], const uint32_t in[8], uint32_t should_negate);
    fn P256_negate_mod_n_if(out: *mut u32, inn: *const u32, should_negate: u32);

    fn P256_check_range_n(a: &[u32; 8]) -> bool;
    fn P256_check_range_p(a: &[u32; 8]) -> bool;
}

const ONE_MONTGOMERY: [u32; 8] = [1, 0, 0, 0xffffffff, 0xffffffff, 0xffffffff, 0xfffffffe, 0];

// This table contains 1G, 3G, 5G, ... 15G in affine coordinates in montgomery form
#[rustfmt::skip]
static P256_BASEPOINT_PRECOMP: [[[u32; 8]; 2]; 8]= [
[[0x18a9143c, 0x79e730d4, 0x5fedb601, 0x75ba95fc, 0x77622510, 0x79fb732b, 0xa53755c6, 0x18905f76],
[0xce95560a, 0xddf25357, 0xba19e45c, 0x8b4ab8e4, 0xdd21f325, 0xd2e88688, 0x25885d85, 0x8571ff18]],
[[0x4eebc127, 0xffac3f90, 0x87d81fb, 0xb027f84a, 0x87cbbc98, 0x66ad77dd, 0xb6ff747e, 0x26936a3f],
[0xc983a7eb, 0xb04c5c1f, 0x861fe1a, 0x583e47ad, 0x1a2ee98e, 0x78820831, 0xe587cc07, 0xd5f06a29]],
[[0xc45c61f5, 0xbe1b8aae, 0x94b9537d, 0x90ec649a, 0xd076c20c, 0x941cb5aa, 0x890523c8, 0xc9079605],
[0xe7ba4f10, 0xeb309b4a, 0xe5eb882b, 0x73c568ef, 0x7e7a1f68, 0x3540a987, 0x2dd1e916, 0x73a076bb]],
[[0xa0173b4f, 0x746354e, 0xd23c00f7, 0x2bd20213, 0xc23bb08, 0xf43eaab5, 0xc3123e03, 0x13ba5119],
[0x3f5b9d4d, 0x2847d030, 0x5da67bdd, 0x6742f2f2, 0x77c94195, 0xef933bdc, 0x6e240867, 0xeaedd915]],
[[0x264e20e8, 0x75c96e8f, 0x59a7a841, 0xabe6bfed, 0x44c8eb00, 0x2cc09c04, 0xf0c4e16b, 0xe05b3080],
[0xa45f3314, 0x1eb7777a, 0xce5d45e3, 0x56af7bed, 0x88b12f1a, 0x2b6e019a, 0xfd835f9b, 0x86659cd]],
[[0x6245e404, 0xea7d260a, 0x6e7fdfe0, 0x9de40795, 0x8dac1ab5, 0x1ff3a415, 0x649c9073, 0x3e7090f1],
[0x2b944e88, 0x1a768561, 0xe57f61c8, 0x250f939e, 0x1ead643d, 0xc0daa89, 0xe125b88e, 0x68930023]],
[[0x4b2ed709, 0xccc42563, 0x856fd30d, 0xe356769, 0x559e9811, 0xbcbcd43f, 0x5395b759, 0x738477ac],
[0xc00ee17f, 0x35752b90, 0x742ed2e3, 0x68748390, 0xbd1f5bc1, 0x7cd06422, 0xc9e7b797, 0xfbc08769]],
[[0xbc60055b, 0x72bcd8b7, 0x56e27e4b, 0x3cc23ee, 0xe4819370, 0xee337424, 0xad3da09, 0xe2aa0e43],
[0x6383c45d, 0x40b8524f, 0x42a41b25, 0xd7663554, 0x778a4797, 0x64efa6de, 0x7079adf4, 0x2042170a]]
];

// This contains two tables, 8 points each in affine coordinates in montgomery form
// The first table contains these points:
// (2^192 - 2^128 - 2^64 - 1)G
// (2^192 - 2^128 - 2^64 + 1)G
// (2^192 - 2^128 + 2^64 - 1)G
// (2^192 - 2^128 + 2^64 + 1)G
// (2^192 + 2^128 - 2^64 - 1)G
// (2^192 + 2^128 - 2^64 + 1)G
// (2^192 + 2^128 + 2^64 - 1)G
// (2^192 + 2^128 + 2^64 + 1)G
// The second table contains the same points multiplied by 2^32
#[rustfmt::skip]
const P256_BASEPOINT_PRECOMP2: [[[[u32; 8]; 2]; 8]; 2] =
[
[
[[0x670844e0, 0x52d8a7c9, 0xef68a29d, 0xe33bdc, 0x4bdb7361, 0xf3d2848, 0x91c5304d, 0x5222c821],
[0xdf73fc25, 0xea6d2944, 0x255c81b, 0xa04c0f55, 0xefe488a8, 0x29acdc97, 0x80a560de, 0xbe2e158f]],
[[0x2b13e673, 0xfc8511ee, 0xd103ed24, 0xffc58dee, 0xea7e99b8, 0x1022523a, 0x4afc8a17, 0x8f43ea39],
[0xc5f33d0b, 0x8f4e2dbc, 0xd0aa1681, 0x3bc099fa, 0x79ff9df1, 0xffbb7b41, 0xd58b57c4, 0x180de09d]],
[[0x8bd1cda5, 0x56430752, 0x8e05eda5, 0x1807577f, 0x956896e9, 0x99c699b, 0xf1f0efb5, 0x83d6093d],
[0xed97061c, 0xef5af17e, 0x30d4c3c, 0x35b977b8, 0x49229439, 0x81fa75a2, 0xa0b6d35d, 0xf5a22070]],
[[0x74f81cf1, 0x814c5365, 0x120065b, 0xe30baff7, 0x15132621, 0x80ae1256, 0x36a80788, 0x16d2b8cb],
[0xecc50bca, 0x33d14697, 0x17aedd21, 0x19a9dfb0, 0xedc3f766, 0x523fbcc7, 0xb2cf5afd, 0x9c4de6dd]],
[[0xcf0d9f6d, 0x5305a9e6, 0x81a9b021, 0x5839172f, 0x75c687cf, 0xcca7a4dd, 0x844be22f, 0x36d59b3e],
[0x111a53e9, 0xcace7e62, 0xf063f3a1, 0x91c843d4, 0xda812da, 0xbf77e5f0, 0x437f3176, 0xe64af9c]],
[[0xcf07517d, 0xdbd568bb, 0xba6830b9, 0x2f1afba2, 0xe6c4c2a6, 0x15b6807c, 0xe4966aef, 0x91c7eabc],
[0xd6b2b6e6, 0x716dea1b, 0x19f85b4b, 0x248c43d1, 0x4a315e2a, 0x16dcfd60, 0xc72b3d0b, 0x15fdd303]],
[[0x42b7dfd5, 0xe40bf9f4, 0x2d934f2a, 0x673689f3, 0x30a6f50b, 0x8314beb4, 0x976ec64e, 0xd17af2bc],
[0x1ee7ddf1, 0x39f66c4f, 0x68ea373c, 0x7f68e18b, 0x53d0b186, 0x5166c1f2, 0x7be58f14, 0x95dda601]],
[[0x42913074, 0xd5ae356, 0x48a542b1, 0x55491b27, 0xb310732a, 0x469ca665, 0x5f1a4cc1, 0x29591d52],
[0xb84f983f, 0xe76f5b6b, 0x9f5f84e1, 0xbe7eef41, 0x80baa189, 0x1200d496, 0x18ef332c, 0x6376551f]]
],
[
[[0x7c4e54f5, 0xb9e5cbc0, 0xe1410e34, 0xc53a1a17, 0xec454425, 0x3e199130, 0x1700902e, 0xb029c97e],
[0x786423b6, 0x2de66e11, 0xb41a95be, 0x262dc914, 0x451b683, 0x51766abd, 0x85bb6fb1, 0x55ad5f34]],
[[0x9066cb79, 0x74f4f1c, 0x30c8b94e, 0x1ab31bd6, 0xd74275b3, 0x6d3f012f, 0x9ddcce40, 0xa214d0b1],
[0xd165050a, 0x24aedf74, 0xe0e5dc3e, 0x95f17ece, 0xd9224456, 0x6ada9cda, 0x2dd60eea, 0x1fadb2d1]],
[[0xe20cfb9b, 0xa3d83091, 0xba76e0cb, 0xae79c975, 0xc8858a6e, 0xa5f2a588, 0x874a3168, 0xe897a5f4],
[0x7d48f096, 0xf6c1ef40, 0xc35b132c, 0x1f9c516b, 0x53c479fd, 0xe1040f91, 0x9df06743, 0x60e881f]],
[[0x52a90e51, 0x9e0ad72, 0x38c50a96, 0xb7e66ea3, 0x7d997770, 0xab32ad05, 0x445671cb, 0xceaffe2],
[0x5d37cc99, 0xdfbe753c, 0xe0fea2d5, 0x95d068cc, 0x4dd77cb6, 0x1e37cdda, 0x55530688, 0x88c5a4bb]],
[[0xc7744f1, 0x3413f033, 0xbc816702, 0x23c05c89, 0x1192b5ac, 0x2322ee9a, 0x373180bb, 0xc1636a0],
[0xbdde0207, 0xfe2f3d4, 0xc23578d8, 0xe1a093a, 0xc888ead, 0x6e5f0d1, 0x52a2b660, 0x9ca285a5]],
[[0xce923964, 0xdae76995, 0xa34c7993, 0xcc96493a, 0xea73d9e7, 0xd19b5144, 0x311e6e34, 0x4a5c263],
[0xd9a2a443, 0x7db5b32b, 0x2cfd960c, 0x3754bd33, 0xa430f15, 0xc5bcc98, 0xd9a94574, 0x5651201f]],
[[0xfc0418fe, 0xebdd8921, 0x34e20036, 0x37015b39, 0xdf03a353, 0xcf4fcd8f, 0xf12cab16, 0xdc2de6e1],
[0xd071df14, 0x9c17cc1a, 0x63415530, 0xd7c5e6a3, 0x68f3fb1e, 0xb5301660, 0x18269301, 0xb5f70bc9]],
[[0x79ec1a0f, 0x2d8daefd, 0xceb39c97, 0x3bbcd6fd, 0x58f61a95, 0xf5575ffc, 0xadf7b420, 0xdbd986c4],
[0x15f39eb7, 0x81aa8814, 0xb98d976c, 0x6ee2fcf5, 0xcf2f717d, 0x5465475d, 0x6860bbd0, 0x8e24d3c4]]
]
];

// Constant time abs
// but not really abs, only works for +/-15
#[inline(always)]
fn abs_int(a: i8) -> u32 {
    let a_u: u32 = a as i32 as u32;
    let mut mask: u32 = a_u >> 31;
    mask |= mask << 1;
    mask |= mask << 2;
    let mut result: u32 = ((-a) as u32) & mask;
    result |= (a as u32) & (mask ^ 0xF);
    result
}

/// Checks that the argument, as little-endian integer,
/// is a reduced non-zero element of the scalar field.
///
/// In other words, that it is in the range `1..=n-1`,
/// where `n = 2^256 - 2^224 + 2^192 - 0x4319055258e8617b0c46353d039cdaaf`.
#[inline]
#[must_use]
pub fn check_range_n(a: &[u32; 8]) -> bool {
    unsafe { P256_check_range_n(a) }
}

/// Checks that the argument, as little-endian integer,
/// is a reduced element of the base field.
///
/// In other words, that it is in the range `0..=p-1`,
/// where `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`.
#[inline]
#[must_use]
pub fn check_range_p(a: &[u32; 8]) -> bool {
    unsafe { P256_check_range_p(a) }
}

/// Converts endianness by reversing the input value.
///
/// The output and input pointers may NOT refer to the same location
/// and have no alignment requirements.
pub fn convert_endianness(output: &mut [u8; 32], input: &[u8; 32]) {
    (0..16).for_each(|i| {
        let t: u8 = input[31 - i];
        output[31 - i] = input[i];
        output[i] = t;
    });
}

fn u32x8_to_u8x32(input: &[u32; 8]) -> &[u8; 32] {
    unsafe { core::mem::transmute::<&[u32; 8], &[u8; 32]>(input) }
}

fn u32x8_to_u8x32_mut(input: &mut [u32; 8]) -> &mut [u8; 32] {
    unsafe { core::mem::transmute::<&mut [u32; 8], &mut [u8; 32]>(input) }
}

/// Uncompressed encoding
///
/// `04 || Px || Py`.
pub fn point_to_octet_string_uncompressed(out: &mut [u8; 65], x: &[u32; 8], y: &[u32; 8]) {
    out[0] = 4;
    convert_endianness((&mut out[1..33]).try_into().unwrap(), u32x8_to_u8x32(x));
    convert_endianness((&mut out[33..65]).try_into().unwrap(), u32x8_to_u8x32(y));
}

/// Compressed encoding
///
/// `02 || Px` if Py is even and `03 || Px` if Py is odd.
pub fn point_to_octet_string_compressed(out: &mut [u8; 33], x: &[u32; 8], y: &[u32; 8]) {
    out[0] = 2_u8.wrapping_add((y[0] & 1) as u8);
    convert_endianness((&mut out[1..]).try_into().unwrap(), u32x8_to_u8x32(x))
}

/// Hybrid encoding
///
/// `06 || Px || Py` if Py is even and `07 || Px || Py` if Py is odd
/// (a pretty useless encoding).
pub fn point_to_octet_string_hybrid(out: &mut [u8; 65], x: &[u32; 8], y: &[u32; 8]) {
    out[0] = 6_u8.wrapping_add((y[0] & 1) as u8);
    convert_endianness((&mut out[1..33]).try_into().unwrap(), u32x8_to_u8x32(x));
    convert_endianness((&mut out[33..65]).try_into().unwrap(), u32x8_to_u8x32(y));
}

/// Decodes a point according to the three encodings above.
///
/// include_p256_decode_point: first byte is "04", "06" or "07" and input length is 65 bytes
/// include_p256_decompress_point: first byte is "02" or "03" and input length is 33 bytes
///
/// Returns true if the input string confirms to a valid encoding and the point lies on the curve,
/// otherwise false.
///
/// NOTE: The return value MUST be checked in case the point is not guaranteed to lie on the curve (e.g. if it
/// is received from an untrusted party).
#[must_use]
pub fn octet_string_to_point(x: &mut [u32; 8], y: &mut [u32; 8], input: &[u8]) -> bool {
    if let Ok(slice) = input[1..33].try_into() {
        convert_endianness(u32x8_to_u8x32_mut(x), slice);
        if unsafe { !P256_check_range_p(x) } {
            return false;
        }

        if (input[0] == 4 || ((input[0] >> 1) == 3)) && input.len() == 65 {
            convert_endianness(u32x8_to_u8x32_mut(y), input[33..65].try_into().unwrap());
            if unsafe { !P256_check_range_p(y) } {
                return false;
            }

            if (input[0] >> 1) == 3 && u32::from(input[0] & 1) != (y[0] & 1) {
                return false;
            }

            let mut x_mont: [u32; 8] = [0; 8];
            let mut y_mont: [u32; 8] = [0; 8];

            unsafe { P256_to_montgomery(&raw mut x_mont as _, x) };
            unsafe { P256_to_montgomery(&raw mut y_mont as _, y) };
            unsafe { P256_point_is_on_curve(&raw const x_mont as _, &raw const y_mont as _) }
        } else if (input[0] >> 1) == 1 && input.len() == 33 {
            unsafe {
                P256_decompress_point(
                    &raw mut *y as _,
                    &raw const *x as _,
                    u32::from(input[0] & 1),
                )
            }
        } else {
            false
        }
    } else {
        false
    }
}

// Calculates scalar*P in constant time (except for the scalars 2 and n-2, for which the results take a few extra cycles to compute)
fn scalarmult_variable_base(
    output_mont_x: &mut [u32; 8],
    output_mont_y: &mut [u32; 8],
    scalar: &[u32; 8],
) {
    // Based on https://eprint.iacr.org/2014/130.pdf, Algorithm 1.

    // The algorithm used requires the scalar to be odd. If even, negate the scalar modulo p to make it odd, and later negate the end result.
    let mut scalar2: [u32; 8] = [0; 8];
    let even: u32 = ((scalar[0]) & 1) ^ 1;
    unsafe { P256_negate_mod_n_if(scalar2.as_mut_ptr(), scalar.as_ptr(), even) };

    // Rewrite the scalar as e[0] + 2^4*e[1] + 2^8*e[2] + ... + 2^252*e[63], where each e[i] is an odd number and -15 <= e[i] <= 15.
    let mut e: [i8; 64] = [0; 64];
    e[0] = (scalar2[0] & 0xf) as i8;
    (1..64).for_each(|i| {
        // Extract 4 bits
        e[i] = ((scalar2[i / 8] >> ((i % 8) * 4)) & 0xf) as u8 as i8;
        // If even, subtract 2^4 from e[i - 1] and add 1 to e[i]
        e[i - 1] -= ((e[i] & 1) ^ 1) << 4;
        e[i] |= 1;
    });

    // Create a table of P, 3P, 5P, ... 15P.
    let mut table: [[[u32; 8]; 3]; 8] = [[[0; 8]; 3]; 8];
    table[0][0].copy_from_slice(output_mont_x);
    table[0][1].copy_from_slice(output_mont_y);
    table[0][2].copy_from_slice(&ONE_MONTGOMERY);
    unsafe { P256_double_j(&raw mut table[7] as _, &raw const table[0] as _) };
    (1..8).for_each(|i| {
        table.copy_within(7..8, i);
        unsafe {
            P256_add_sub_j(
                &raw mut table[i] as _,
                &raw const table[i - 1] as _,
                false,
                false,
            )
        };
    });

    // Calculate the result as (((((((((e[63]*G)*2^4)+e[62])*2^4)+e[61])*2^4)...)+e[1])*2^4)+e[0] = (2^252*e[63] + 2^248*e[62] + ... + e[0])*G.

    let mut current_point: [[u32; 8]; 3] = [[0; 8]; 3];

    // e[63] is never negative
    current_point.copy_from_slice(&table[(e[63] >> 1) as u8 as usize]);

    (0..63).rev().for_each(|i| {
        (0..4).for_each(|_| unsafe {
            P256_double_j(&raw mut current_point as _, &raw const current_point as _)
        });

        let mut selected_point: [[u32; 8]; 3] = [[0; 8]; 3];
        selected_point.copy_from_slice(&table[(abs_int(e[i]) >> 1) as u8 as usize]);

        unsafe {
            P256_negate_mod_p_if(
                selected_point[1].as_mut_ptr(),
                selected_point[1].as_ptr(),
                ((e[i] as u8) >> 7) as u32,
            )
        };

        // There is (only) one odd input scalar that leads to an exception when i == 0: n-2,
        // in that case current_point will be equal to selected_point and hence a doubling
        // will occur instead. We don't bother fixing the same constant time for that case since
        // the probability of that random value to be generated is around 1/2^255 and an
        // attacker could easily test this case anyway.
        unsafe {
            P256_add_sub_j(
                &raw mut current_point as _,
                &raw const selected_point as _,
                false,
                false,
            )
        };
    });
    unsafe {
        P256_jacobian_to_affine(
            output_mont_x.as_mut_ptr(),
            output_mont_y.as_mut_ptr(),
            current_point.as_ptr(),
        )
    };

    // If the scalar was initially even, we now negate the result to get the correct result, since -(scalar*G) = (-scalar*G).
    // This is done by negating y, since -(x,y) = (x,-y).
    unsafe { P256_negate_mod_p_if(output_mont_y.as_mut_ptr(), output_mont_y.as_mut_ptr(), even) };
}

#[must_use]
fn scalarmult_generic_no_scalar_check(
    output_mont_x: &mut [u32; 8],
    output_mont_y: &mut [u32; 8],
    scalar: &[u32; 8],
    in_x: &[u32; 8],
    in_y: &[u32; 8],
) -> bool {
    if unsafe { !P256_check_range_p(in_x) || !P256_check_range_p(in_y) } {
        false
    } else {
        unsafe { P256_to_montgomery(&raw mut *output_mont_x as _, in_x) };
        unsafe { P256_to_montgomery(&raw mut *output_mont_y as _, in_y) };

        if unsafe {
            !P256_point_is_on_curve(
                &raw const *output_mont_x as _,
                &raw const *output_mont_y as _,
            )
        } {
            false
        } else {
            scalarmult_variable_base(output_mont_x, output_mont_y, scalar);
            true
        }
    }
}

/// Raw scalar multiplication by any point on the elliptic curve.
///
/// This function can be used to implement custom algorithms using the P-256 curve.
///
/// This function validates all inputs and proceeds only if the scalar is within the range 1 to n-1, where n
/// is the order of the elliptic curve, and the input point's coordinates are each less than the order of
/// the prime field. If validation succeeds, true is returned. Otherwise false is returned.
#[must_use]
pub fn scalarmult_generic(
    result_x: &mut [u32; 8],
    result_y: &mut [u32; 8],
    scalar: &[u32; 8],
    in_x: &[u32; 8],
    in_y: &[u32; 8],
) -> bool {
    if unsafe {
        !P256_check_range_n(scalar)
            || !scalarmult_generic_no_scalar_check(result_x, result_y, scalar, in_x, in_y)
    } {
        false
    } else {
        unsafe { P256_from_montgomery(result_x, &raw const *result_x as _) };
        unsafe { P256_from_montgomery(result_y, &raw const *result_y as _) };
        true
    }
}

/// Generates the shared secret according to the ECDH standard.
///
/// The shared secret parameter will contain the big endian encoding for the x coordinate of the scalar
/// multiplication of the private key and the input point (other's public key), if the function succeeds.
///
/// If the other's public key point does not lie on the curve, this function fails and false is returned.
/// Otherwise, shared secret is calculated and true is returned.
///
/// NOTE: The return value MUST be checked since the other's public key point cannot generally be trusted.
#[must_use]
pub fn ecdh_calc_shared_secret(
    shared_secret: &mut [u8; 32],
    private_key: &[u32; 8],
    others_public_key_x: &[u32; 8],
    others_public_key_y: &[u32; 8],
) -> bool {
    let mut result_x: [u32; 8] = [0; 8];
    let mut result_y: [u32; 8] = [0; 8];
    if !scalarmult_generic_no_scalar_check(
        &mut result_x,
        &mut result_y,
        private_key,
        others_public_key_x,
        others_public_key_y,
    ) {
        false
    } else {
        unsafe { P256_from_montgomery(&raw mut result_x as _, &raw const result_x as _) };
        convert_endianness(shared_secret, u32x8_to_u8x32(&result_x));
        true
    }
}

/// Calculates the public key from a given private key for use by either ECDSA or ECDH.
///
/// The private key shall be taken from a random value that MUST have been generated by a cryptographically
/// secure random number generator that generates 256 random bits. This function validates that the private key
/// lies in the accepted range 1 to n-1, where n is the order of the elliptic curve, and returns true only if
/// this validation succeeds. If random value is out of that range, false is returned and in this case a new
/// random value needs to be generated and this function MUST be called again until true is returned.
///
/// The public key is created by performing a scalar multiplication of the private key and the base point of
/// the curve.
///
/// Only use a keypair for either ECDSA or ECDH, not both, and don't use the private key for any other purposes.
#[must_use]
pub fn keygen(
    public_key_x: &mut [u32; 8],
    public_key_y: &mut [u32; 8],
    private_key: &[u32; 8],
) -> bool {
    scalarmult_base(public_key_x, public_key_y, private_key)
}

macro_rules! get_bit {
    ($arr:ident, $i:expr) => {
        (($arr[$i / 32] >> ($i % 32)) & 1)
    };
}

// Calculates scalar*G in constant time
fn scalarmult_fixed_base(
    output_mont_x: &mut [u32; 8],
    output_mont_y: &mut [u32; 8],
    scalar: &[u32; 8],
) {
    let mut scalar2: [u32; 8] = [0; 8];

    // Just as with the algorithm used in variable base scalar multiplication, this algorithm requires the scalar to be odd.
    let even: u32 = ((scalar[0]) & 1) ^ 1;
    unsafe { P256_negate_mod_n_if(scalar2.as_mut_ptr(), scalar.as_ptr(), even) };

    // This algorithm conceptually rewrites the odd scalar as s[0] + 2^1*s[1] + 2^2*s[2] + ... + 2^255*s[255], where each s[i] is -1 or 1.
    // By initially setting s[i] to the corresponding bit S[i] in the original odd scalar S, we go from lsb to msb, and whenever a value s[i] is 0,
    // increase s[i] by 1 and decrease s[i-1] by 2.
    // This will result in that s[i] = S[i+1] == 1 ? 1 : -1 for i < 255, and s[255] = 1.

    // We then form the scalars abs(s[j] + s[j+64]*2^64 + s[j+128]*2^128 + s[j+192]*2^192)*(2^32 * floor(j / 32)) for different 0 <= j < 64.
    // Each scalar times G has already been precomputed in p256_basepoint_precomp2.
    // That way we only need 31 point doublings and 63 point additions.

    let mut current_point: [[u32; 8]; 3] = [[0; 8]; 3];
    let mut selected_point: [[u32; 8]; 2] = [[0; 8]; 2];

    for i in (0..32).rev() {
        {
            let mut mask: u32 = get_bit!(scalar2, i + 32 + 1)
                | (get_bit!(scalar2, i + 64 + 32 + 1) << 1)
                | (get_bit!(scalar2, i + 2 * 64 + 32 + 1) << 2);
            if i == 31 {
                current_point[..2].copy_from_slice(&P256_BASEPOINT_PRECOMP2[1][mask as usize]);
                current_point[2].copy_from_slice(&ONE_MONTGOMERY);
            } else {
                unsafe { P256_double_j(&raw mut current_point as _, &raw mut current_point as _) };

                let sign: u32 = get_bit!(scalar2, i + 3 * 64 + 32 + 1).wrapping_sub(1); // positive: 0, negative: -1
                mask = (mask ^ sign) & 7;
                selected_point.copy_from_slice(&P256_BASEPOINT_PRECOMP2[1][mask as usize]);
                unsafe {
                    P256_negate_mod_p_if(
                        selected_point[1].as_mut_ptr(),
                        selected_point[1].as_ptr(),
                        sign & 1,
                    )
                };
                unsafe {
                    P256_add_sub_j(
                        &raw mut current_point as _,
                        &raw const selected_point as _,
                        false,
                        true,
                    )
                };
            }
        }
        {
            let mut mask: u32 = get_bit!(scalar2, i + 1)
                | (get_bit!(scalar2, i + 64 + 1) << 1)
                | (get_bit!(scalar2, i + 2 * 64 + 1) << 2);
            let sign: u32 = get_bit!(scalar2, i + 3 * 64 + 1).wrapping_sub(1); // positive: 0, negative: -1
            mask = (mask ^ sign) & 7;
            selected_point.copy_from_slice(&P256_BASEPOINT_PRECOMP2[0][mask as usize]);
            unsafe {
                P256_negate_mod_p_if(
                    selected_point[1].as_mut_ptr(),
                    selected_point[1].as_ptr(),
                    sign & 1,
                )
            };
            unsafe {
                P256_add_sub_j(
                    &raw mut current_point as _,
                    &raw const selected_point as _,
                    false,
                    true,
                )
            };
        }
    }

    unsafe {
        P256_jacobian_to_affine(
            output_mont_x.as_mut_ptr(),
            output_mont_y.as_mut_ptr(),
            current_point.as_ptr(),
        )
    };

    // Negate final result if the scalar was initially even.
    unsafe { P256_negate_mod_p_if(output_mont_y.as_mut_ptr(), output_mont_y.as_mut_ptr(), even) };
}

/// Raw scalar multiplication by the base point of the elliptic curve.
///
/// This function can be used to implement custom algorithms using the P-256 curve.
///
/// This function validates that the scalar lies in the accepted range 1 to n-1, where n is the order of the
/// elliptic curve, and returns true only if this validation succeeds. Otherwise false is returned.
#[must_use]
pub fn scalarmult_base(
    result_x: &mut [u32; 8],
    result_y: &mut [u32; 8],
    scalar: &[u32; 8],
) -> bool {
    if unsafe { !P256_check_range_n(scalar) } {
        false
    } else {
        scalarmult_fixed_base(result_x, result_y, scalar);
        unsafe { P256_from_montgomery(result_x, &raw const *result_x as _) };
        unsafe { P256_from_montgomery(result_y, &raw const *result_y as _) };
        true
    }
}

/// Sign precomputation state.
///
/// The content shall be treated as opaque to the API user and shall not be inspected or modified.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct SignPrecomp {
    pub r: [u32; 8],
    pub k_inv: [u32; 8],
}

/// Creates an ECDSA signature.
///
/// The parameter "k" shall consist of a 256-bit random integer value. This random value MUST be generated from
/// a cryptographically secure random number generator, and MUST be unique for every pair of message hash and
/// private key.
///
/// With a small probability (~ 2^-32), this function will fail and return false for the given "k" and this
/// function MUST in that case be called again with a new random "k", until true is returned. This is in line
/// with the ECDSA standard.
///
/// As an alternative to using a random "k", "k" might be derived deterministically from the input, using a
/// sophisticated hash construction such as RFC 6979, or e.g. by hashing the private key, message hash and a
/// retry counter, using a secure hash function such as SHA-256.
#[must_use]
pub fn sign(
    r: &mut [u32; 8],
    s: &mut [u32; 8],
    hash: &[u8],
    private_key: &[u32; 8],
    k: &[u32; 8],
) -> bool {
    let mut t: SignPrecomp = Default::default();
    if !sign_step1(&mut t, k) {
        r.fill(0);
        s.fill(0);
        false
    } else {
        sign_step2(r, s, hash, private_key, &mut t)
    }
}

/// Creates an ECDSA signature, using a two-step procedure.
///
/// This function performs the first of two steps, and accounts for 99% of the time spent for generating an
/// ECDSA signature.
///
/// By splitting up into two steps, most of the work could be spent before deciding what message to sign, or
/// which private key to use.
///
/// The parameter "k" shall consist of a 256-bit random integer value. This random value MUST be generated from
/// a cryptographically secure random number generator, and MUST be unique for every pair of message hash and
/// private key.
///
/// With a small probability (~ 2^-32), this function will fail and return false for the given "k" and this
/// function MUST in that case be called again with a new random "k", until true is returned. This is in line
/// with the ECDSA standard.
///
/// As an alternative to using a random "k", "k" might be derived deterministically from the input, using a
/// sophisticated hash construction such as RFC 6979, or e.g. by hashing the private key, message hash and a
/// retry counter, using a secure hash function such as SHA-256.
///
/// The "result" parameter will contain the computed state, that is later to be passed to p256_sign_step2.
/// A result state MUST NOT be reused for generating multiple signatures.
#[must_use]
pub fn sign_step1(result: &mut SignPrecomp, k: &[u32; 8]) -> bool {
    #[allow(clippy::never_loop)]
    loop {
        if unsafe { !P256_check_range_n(k) } {
            break;
        }
        let mut output_x: [u32; 8] = [0; 8];
        let mut output_y: [u32; 8] = [0; 8];
        scalarmult_fixed_base(&mut output_x, &mut output_y, k);
        mod_n_inv(&mut result.k_inv, k);
        unsafe { P256_from_montgomery(&raw mut result.r, &raw const output_x as _) };
        unsafe { P256_reduce_mod_n_32bytes(result.r.as_mut_ptr(), result.r.as_ptr()) };

        let r_sum: u32 = (0..8).fold(0, |r_sum, i| r_sum | result.r[i]);
        if r_sum == 0 {
            break;
        }
        return true;
    }

    result.r.fill(0);
    result.k_inv.fill(0);
    false
}

// Takes the leftmost 256 bits in hash (treated as big endian),
// and converts to little endian integer z.
fn hash_to_z(z: &mut [u8], hash: &[u8]) {
    let hashlen: usize = core::cmp::min(hash.len(), 32);
    (0..hashlen).for_each(|i| {
        z[i] = hash[hashlen - 1 - i];
    });
    z[hashlen..].fill(0);
}

/// Second step of creating an ECDSA signature, using a two-step procedure.
///
/// This function performs the second of two steps, and accounts for the last 1% of the time spent for generating
/// an ECDSA signature.
///
/// The "sign_precomp" parameter shall contain a pointer to a state generated by p256_sign_step1.
///
/// With a small probability (~ 2^-256), this function will fail, due to the given "k" from the first step is
/// not compatible with the rest of the input, and return false. In this case, the procedure MUST be started
/// over from step 1 with a new random "k".  This is in line with the ECDSA standard. Otherwise true is returned
/// and the signature is placed in "r" and "s".
///
/// When this function returns, "sign_precomp" is also zeroed out and may hence not be reused.
#[must_use]
pub fn sign_step2(
    r: &mut [u32; 8],
    s: &mut [u32; 8],
    hash: &[u8],
    private_key: &[u32; 8],
    sign_precomp: &mut SignPrecomp,
) -> bool {
    #[allow(clippy::never_loop)]
    loop {
        // just make sure user did not input an obviously invalid precomp
        if unsafe {
            !P256_check_range_n(&sign_precomp.k_inv) || !P256_check_range_n(&sign_precomp.r)
        } {
            break;
        }
        hash_to_z(u32x8_to_u8x32_mut(r), hash);
        unsafe {
            P256_mul_mod_n(
                s.as_mut_ptr(),
                sign_precomp.r.as_ptr(),
                private_key.as_ptr(),
            )
        };
        unsafe { P256_add_mod_n(s.as_mut_ptr(), r as *const u32, s.as_ptr()) };
        unsafe { P256_mul_mod_n(s.as_mut_ptr(), sign_precomp.k_inv.as_ptr(), s.as_ptr()) };

        r.copy_from_slice(&sign_precomp.r);

        let s_sum: u32 = s.iter().fold(0, |s_sum, s| s_sum | s);
        if s_sum == 0 {
            break;
        }
        sign_precomp.r.fill(0);
        sign_precomp.k_inv.fill(0);
        return true;
    }

    r.fill(0);
    s.fill(0);
    false
}

// Creates a representation of a (little endian integer),
// so that r[0] + 2*r[1] + 2^2*r[2] + 2^3*r[3] + ... = a,
// where each r[i] is -15, -13, ..., 11, 13, 15 or 0.
// Only around 1/5.5 of the r[i] will be non-zero.
fn slide_257(a: &[u8; 32]) -> [i8; 257] {
    let mut r: [i8; 257] = [0; 257];
    (0..256).for_each(|i| {
        r[i] = (1 & (a[i >> 3] >> (i & 7))) as i8;
    });
    r[256] = 0;

    (0..256).for_each(|i| {
        if r[i] != 0 {
            let mut b: usize = 1;
            while b <= 4 && i + b < 256 {
                if r[i + b] != 0 {
                    if r[i] + (r[i + b] << b) <= 15 {
                        r[i] += r[i + b] << b;
                        r[i + b] = 0;
                    } else if r[i] - (r[i + b] << b) >= -15 {
                        r[i] -= r[i + b] << b;
                        loop {
                            r[i + b] = 0;
                            b += 1;
                            if r[i + b] == 0 {
                                r[i + b] = 1;
                                b -= 1; // Will be added back after loop footer b++
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                b += 1;
            }
        }
    });

    r
}

/// Verifies an ECDSA signature.
///
/// Returns true if the signature is valid for the given input, otherwise false.
#[must_use = "The return value indicates if the message is authentic"]
pub fn verify(
    public_key_x: &[u32; 8],
    public_key_y: &[u32; 8],
    hash: &[u8],
    r: &[u32; 8],
    s: &[u32; 8],
) -> bool {
    if unsafe { !P256_check_range_n(r) || !P256_check_range_n(s) } {
        return false;
    }

    if unsafe { !P256_check_range_p(public_key_x) || !P256_check_range_p(public_key_y) } {
        return false;
    }

    let mut pk_table: [[[u32; 8]; 3]; 8] = [[[0; 8]; 3]; 8];
    unsafe {
        P256_to_montgomery(&raw mut pk_table[0][0] as _, public_key_x);
        P256_to_montgomery(&raw mut pk_table[0][1] as _, public_key_y);
    }
    pk_table[0][2].copy_from_slice(&ONE_MONTGOMERY);

    if unsafe {
        !P256_point_is_on_curve(
            &raw const pk_table[0][0] as _,
            &raw const pk_table[0][1] as _,
        )
    } {
        return false;
    }

    // Create a table of P, 3P, 5P, ..., 15P, where P is the public key.
    unsafe { P256_double_j(&raw mut pk_table[7] as _, &raw const pk_table[0] as _) };
    (1..8).for_each(|i| {
        pk_table.copy_within(7..8, i);
        unsafe {
            P256_add_sub_j(
                &raw mut pk_table[i] as _,
                &raw const pk_table[i - 1] as _,
                false,
                false,
            )
        };
    });

    let mut z: [u32; 8] = [0; 8];
    hash_to_z(u32x8_to_u8x32_mut(&mut z), hash);

    let mut w: [u32; 8] = [0; 8];
    mod_n_inv(&mut w, s);

    let mut u1: [u32; 8] = [0; 8];
    unsafe { P256_mul_mod_n(u1.as_mut_ptr(), z.as_ptr(), w.as_ptr()) };
    let mut u2: [u32; 8] = [0; 8];
    unsafe { P256_mul_mod_n(u2.as_mut_ptr(), r.as_ptr(), w.as_ptr()) };

    // Each value in these arrays will be an odd integer v, so that -15 <= v <= 15.
    // Around 1/5.5 of them will be non-zero.

    let slide_bp: [i8; 257] = slide_257(u32x8_to_u8x32(&u1));
    let slide_pk: [i8; 257] = slide_257(u32x8_to_u8x32(&u2));

    let mut cp: [[u32; 8]; 3] = [[0; 8]; 3];

    #[allow(clippy::comparison_chain)]
    slide_bp
        .iter()
        .rev()
        .zip(slide_pk.iter().rev())
        .for_each(|(&bp, &pk)| {
            unsafe { P256_double_j(&raw mut cp as _, &raw mut cp as _) };

            if bp > 0 {
                unsafe {
                    P256_add_sub_j(
                        &raw mut cp as _,
                        &raw const P256_BASEPOINT_PRECOMP[(bp / 2) as usize] as _,
                        false,
                        true,
                    )
                };
            } else if bp < 0 {
                unsafe {
                    P256_add_sub_j(
                        &raw mut cp as _,
                        &raw const P256_BASEPOINT_PRECOMP[((-bp) / 2) as usize] as _,
                        true,
                        true,
                    )
                };
            }
            if pk > 0 {
                unsafe {
                    P256_add_sub_j(
                        &raw mut cp as _,
                        &raw const pk_table[(pk / 2) as usize] as _,
                        false,
                        false,
                    )
                };
            } else if pk < 0 {
                unsafe {
                    P256_add_sub_j(
                        &raw mut cp as _,
                        &raw const pk_table[((-pk) / 2) as usize] as _,
                        true,
                        false,
                    )
                };
            }
        });

    unsafe { P256_verify_last_step(&raw const *r, &raw const cp as _) }
}

#[repr(C)]
#[derive(Default)]
struct FGInteger {
    // To get the value this struct represents,
    // interpret signed_value as a two's complement 288-bit little endian integer,
    // and negate if flip_sign is -1
    flip_sign: i32, // 0 or -1
    // of 288 bits, 257 are useful (top 31 bits are sign-extended from bit 256)
    signed_value: [u32; 9],
}

#[repr(C)]
#[derive(Default)]
struct XYInteger {
    // To get the value this struct represents,
    // interpret signed_value as an unsigned 288-bit little endian integer,
    // and negate if flip_sign is -1
    flip_sign: i32,  // 0 or -1
    value: [u32; 8], // unsigned value, 0 <= value < P256_order
}

#[repr(C)]
#[derive(Default)]
struct State {
    fg: [FGInteger; 2],
    xy: [XYInteger; 2],
}

fn mod_n_inv(res: &mut [u32; 8], a: &[u32; 8]) {
    // This function follows the algorithm in section 12.1 of https://gcd.cr.yp.to/safegcd-20190413.pdf.
    // It has been altered in the following ways:
    //   1. Due to 32-bit cpu, we use 24 * 31 iterations instead of 12 * 62.
    //   2. P-256 modulus instead of 2^255-19.
    //      744 iterations are still enough and slightly more than the required 741 (floor((49*256+57)/17)).
    //   3. Step 5 has been corrected to go back to step 2 instead of step 3.
    //   4. The order of the matrix multiplications in step 6 has been changed to (T24*(T23*(T22*(...*(T1*[0, 1]))))),
    //      where [0, 1] is a column vector to make it possible to be able to extract the "top-right corner", v, of T24*T23*...*T1.
    //      The result v will then be contained in the first element of the resulting column vector.

    let mut state: [State; 2] = Default::default();

    state[0].fg[0].flip_sign = 0; // non-negative f
    state[0].fg[0]
        .signed_value
        .copy_from_slice(&asm::P256_ORDER); // f
    state[0].fg[1].flip_sign = 0; // non-negative g
    state[0].fg[1].signed_value[..8].copy_from_slice(a); // g
    state[0].fg[1].signed_value[8] = 0; // upper bits of g are 0

    // We later need a factor 2^-744. The montgomery multiplication gives 2^(24*-32)=2^-768, so multiply the init value (1) by 2^24 here.
    state[0].xy[1].value[0] = 1 << 24;

    let mut delta: i32 = 1;
    (0..24).for_each(|i| {
        // Scaled translation matrix Ti
        let mut matrix: [u32; 4] = [0; 4]; // element range: [-2^30, 2^31] (negative numbers are stored in two's complement form)

        // Decode f and g into two's complement representation and use the lowest 32 bits in the P256_divsteps2_31 calculation
        let negate_f: u32 = state[i % 2].fg[0].flip_sign as u32;
        let negate_g: u32 = state[i % 2].fg[1].flip_sign as u32;
        delta = unsafe {
            P256_divsteps2_31(
                delta,
                (state[i % 2].fg[0].signed_value[0] ^ negate_f).wrapping_sub(negate_f),
                (state[i % 2].fg[1].signed_value[0] ^ negate_g).wrapping_sub(negate_g),
                matrix.as_mut_ptr(),
            )
        };

        // "Jump step", calculates the new f and g values that applies after 31 divstep2 iterations
        unsafe {
            P256_matrix_mul_fg_9(
                matrix[0],
                matrix[1],
                state[i % 2].fg.as_ptr(),
                &mut state[(i + 1) % 2].fg[0],
            )
        };
        unsafe {
            P256_matrix_mul_fg_9(
                matrix[2],
                matrix[3],
                state[i % 2].fg.as_ptr(),
                &mut state[(i + 1) % 2].fg[1],
            )
        };

        // Iterate the result vector
        // Due to montgomery multiplication inside this function, each step also adds a 2^-32 factor
        unsafe {
            crate::asm::P256_matrix_mul_mod_n(
                matrix[0],
                matrix[1],
                &state[i % 2].xy,
                &mut state[(i + 1) % 2].xy[0],
            )
        };
        unsafe {
            crate::asm::P256_matrix_mul_mod_n(
                matrix[2],
                matrix[3],
                &state[i % 2].xy,
                &mut state[(i + 1) % 2].xy[1],
            )
        };
    });

    // Calculates val^-1 = sgn(f) * v * 2^-744, where v is the "top-right corner" of the resulting T24*T23*...*T1 matrix.
    // In this implementation, at this point x contains v * 2^-744.
    unsafe {
        P256_negate_mod_n_if(
            res.as_mut_ptr(),
            &state[0].xy[0].value[0],
            ((state[0].xy[0].flip_sign
                ^ state[0].fg[0].flip_sign
                ^ (state[0].fg[0].signed_value[8] as i32))
                & 1) as u32,
        )
    };
}
