// 0x3EFFFFFF -> 1 bit lower than 0.5_f32  (0.5 - 0.25 * f32::EPSILON)
const F32_ROUND: u32 = 0x3EFFFFFF;

// 0x3FDFFFFFFFFFFFFF -> 1 bit lower than 0.5_f64 (0.5 - 0.25 * f64::EPSILON)
const F64_ROUND: u64 = 0x3FDFFFFFFFFFFFFF;

/// Returns the given `f32` as rounded to the nearest integer. Same output as f32::round().
/// This follows the `as` rules for saturated casting of `f32` to `i32`. See more in the Rust 
/// [Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#semantics).
#[inline(always)]
pub fn fast_inline_round_f32(x: f32) -> i32 {
    (x + f32::from_bits((x.to_bits() & 1_u32 << 31) | F32_ROUND)) as i32
}

/// Returns the given `f64` as rounded to the nearest integer. Same output as f64::round().
/// This follows the `as` rules for saturated casting of `f64` to `i64`. See more in the Rust 
/// [Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#semantics).
#[inline(always)]
pub fn fast_inline_round_f64(x: f64) -> i64 {
    (x + f64::from_bits((x.to_bits() & 1_u64 << 63) | F64_ROUND)) as i64
}
