use paste::paste;

use float_quickly::*;

// See: https://github.com/skeeto/hash-prospector
pub fn hash_u32(mut val: u32) -> u32 {
    val = val ^ (val >> 15);
    val = val.wrapping_mul(0x2C1B3C6D);
    val = val ^ (val >> 12);
    val = val.wrapping_mul(0x297A2D39);
    val = val ^ (val >> 15);
    return val;
}

pub fn hash_u64(mut val: u64) -> u64 {
    val = val ^ (val >> 30);
    val = val.wrapping_mul(0xBF58476D1CE4E5B9);
    val = val ^ (val >> 27);
    val = val.wrapping_mul(0x94D049BB133111EB);
    val = val ^ (val >> 31);
    return val;
}

macro_rules! make_generate {
    ($type:ty) => {
        paste! {
            macro_rules! hash {
                ($expr:expr) => {
                    [< hash_ $type >]($expr)
                }
            }

            #[allow(dead_code)]
            fn [< generate_ $type >]() -> Vec<$type> {
                let mut result: Vec<$type> = Vec::with_capacity(256 * 17);
                for idx in 0_u32..256_u32 {
                    let values = [
                        (idx as $type),
                        (hash!(idx as $type)),
                        (hash!(hash!(idx as $type))),
                        (hash!(hash!(hash!(idx as $type)))),
                        (hash!(hash!(hash!(hash!(idx as $type))))),
                        (hash!(hash!(hash!(hash!(hash!(idx as $type)))))),
                        (hash!(hash!(hash!(hash!(hash!(hash!(idx as $type))))))),
                        ((1 as $type).wrapping_shl(idx).wrapping_sub(1)),
                        ((1 as $type).wrapping_shl(idx).wrapping_sub(1)).wrapping_shl(1),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x01_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x02_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x05_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x0A_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x15_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x2A_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0x55_u8 as $type)),
                        ((1 as $type).wrapping_shl(idx).wrapping_mul(0xAA_u8 as $type)),
                    ];
                    result.extend_from_slice(&values);
                }
                return result;
            }
        }
    };
}

make_generate!(u32);
make_generate!(u64);

macro_rules! make_test_brute_force {
    ($float_type:ty, $rounded_type:ty, $hash_type:ty) => {
        paste! {
            #[test]
            fn [< test_brute_force_ $float_type >]() {
                for value in [< generate_ $hash_type >]() {
                    let float = $float_type::from_bits(value);

                    assert_eq!(float.round() as $rounded_type, [< fast_inline_round_ $float_type >](float));
                    assert_eq!((float - $float_type::EPSILON).round() as $rounded_type, [< fast_inline_round_ $float_type >](float - $float_type::EPSILON));
                    assert_eq!((float + $float_type::EPSILON).round() as $rounded_type, [< fast_inline_round_ $float_type >](float + $float_type::EPSILON));
                }
            }
        }
    }
}

make_test_brute_force!(f32, i32, u32);
make_test_brute_force!(f64, i64, u64);

macro_rules! make_test_epsilon_sensitivity {
    ($float_type:ty, $rounded_type:ty) => {
        paste! {
            #[test]
            fn [< test_epsilon_sensitivity_ $float_type >]() {
                assert_ne!(0.5, 0.5 + $float_type::EPSILON);
                assert_ne!(0.5, 0.5 - $float_type::EPSILON);

                assert_ne!((0.5 as $float_type).round(), (0.5 - $float_type::EPSILON).round());
                assert_eq!((0.5 as $float_type).round(), (0.5 + $float_type::EPSILON).round());

                assert_ne!([< fast_inline_round_ $float_type >](0.5), [< fast_inline_round_ $float_type >](0.5 - $float_type::EPSILON));
                assert_eq!([< fast_inline_round_ $float_type >](0.5), [< fast_inline_round_ $float_type >](0.5 + $float_type::EPSILON));

                assert_eq!((0.5 as $float_type).round() as $rounded_type, [< fast_inline_round_ $float_type >](0.5));
                assert_eq!((0.5 - $float_type::EPSILON).round() as $rounded_type, [< fast_inline_round_ $float_type >](0.5 - $float_type::EPSILON));
                assert_eq!((0.5 + $float_type::EPSILON).round() as $rounded_type, [< fast_inline_round_ $float_type >](0.5 + $float_type::EPSILON));
            }
        }
    }
}

make_test_epsilon_sensitivity!(f32, i32);
make_test_epsilon_sensitivity!(f64, i64);
