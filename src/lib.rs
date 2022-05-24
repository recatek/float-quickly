// 0x3EFFFFFF -> 1 bit lower than 0.5_f32  (0.5 - 0.25 * f32::EPSILON)
const F32_ROUND: u32 = 0x3EFFFFFF;
#[inline(always)]
pub fn fast_inline_round_f32(x: f32) -> i32 {
    return (x + f32::from_bits((x.to_bits() & 1_u32 << 32) | F32_ROUND)) as i32;
}

// 0x3FDFFFFFFFFFFFFF -> 1 bit lower than 0.5_f64 (0.5 - 0.25 * f64::EPSILON)
const F64_ROUND: u64 = 0x3FDFFFFFFFFFFFFF;
#[inline(always)]
pub fn fast_inline_round_f64(x: f64) -> i64 {
    return (x + f64::from_bits((x.to_bits() & 1_u64 << 63) | F64_ROUND)) as i64;
}
