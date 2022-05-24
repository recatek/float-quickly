use std::time;

use criterion::*;
use paste::paste;
use libm;

use float_quickly::*;

// See: https://github.com/skeeto/hash-prospector
pub fn hash(mut val: u64) -> u64 {
    val = val ^ (val >> 30);
    val = val.wrapping_mul(0xBF58476D1CE4E5B9);
    val = val ^ (val >> 27);
    val = val.wrapping_mul(0x94D049BB133111EB);
    val = val ^ (val >> 31);
    return val;
}

macro_rules! bench_float {
    ($float_type:ty, $int_type:ty, $round_fn:ident) => {
        |iters| {
            let values: Vec<$float_type> = (0..iters).map(|v| <$float_type>::from_bits(hash(v) as $int_type)).collect();

            let start = time::Instant::now();
            for val in values {
                black_box($round_fn(val));
            }
            return start.elapsed();
        }
    };
}

#[inline(always)]
pub fn rust_round_f32(f: f32) -> i32 {
    f.round() as i32
}

#[inline(always)]
fn rust_round_f64(f: f64) -> i64 {
    f.round() as i64
}

#[inline(always)]
pub fn libm_round_f32(f: f32) -> i32 {
    libm::roundf(f) as i32
}

#[inline(always)]
fn libm_round_f64(f: f64) -> i64 {
    libm::round(f) as i64
}

macro_rules! generate_run_bench_float {
    ($float_type:ty, $int_type:ty) => {
        paste! {
            pub fn [< run_bench_float_ $float_type >](c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!([< run_bench_float_ $float_type >]));
                group.bench_function(stringify!([< run_bench_rust_round_ $float_type >]), move |b| {
                    b.iter_custom(bench_float!($float_type, $int_type, [< rust_round_ $float_type >]));
                });
                group.bench_function(stringify!([< run_bench_libm_round_ $float_type >]), move |b| {
                    b.iter_custom(bench_float!($float_type, $int_type, [< libm_round_ $float_type >]));
                });
                group.bench_function(stringify!([< run_bench_fast_inline_round_ $float_type >]), move |b| {
                    b.iter_custom(bench_float!($float_type, $int_type, [< fast_inline_round_ $float_type >]));
                });
                group.finish();
            }
        }
    };
}

generate_run_bench_float!(f32, u32);
generate_run_bench_float!(f64, u64);

criterion_group! {
    name = bench_float;
    config = Criterion::default().sample_size(2000);
    targets =
        run_bench_float_f32,
        run_bench_float_f64,
}

criterion_main! { bench_float }
