//! Dot-product benchmarks.

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
    Throughput,
};
use multirate::math::dotprod::{dot, dot_lanes};
use num_traits::Zero;
use std::hint::black_box;

/// Slice lengths spanning typical FIR tap counts.
const LENS: [usize; 4] = [16, 64, 256, 1024];

/// Values small enough that an `i32` dot-product cannot overflow.
const I32_RANGE: i32 = 7;

/// Baseline accumulating in an explicit loop, no multiversion dispatch.
fn naive_dot<A, B, Prod>(a: &[A], b: &[B]) -> Prod
where
    A: Copy + std::ops::Mul<B, Output = Prod>,
    B: Copy,
    Prod: Zero,
{
    let mut acc = Prod::zero();
    for (a, b) in a.iter().zip(b.iter()) {
        acc = acc + (*a * *b);
    }
    acc
}

fn f64_samples(len: usize, phase: f64) -> Vec<f64> {
    (0..len).map(|n| (n as f64 * 0.1 + phase).sin()).collect()
}

fn f32_samples(len: usize, phase: f32) -> Vec<f32> {
    (0..len).map(|n| (n as f32 * 0.1 + phase).sin()).collect()
}

fn i32_samples(len: usize, phase: usize) -> Vec<i32> {
    (0..len)
        .map(|n| ((n + phase) % I32_RANGE as usize) as i32 - I32_RANGE / 2)
        .collect()
}

fn bench_type<T>(group: &mut BenchmarkGroup<'_, WallTime>, ty: &str, len: usize, a: &[T], b: &[T])
where
    T: Copy + Zero + std::ops::Mul<T, Output = T> + std::ops::AddAssign,
{
    group.bench_function(BenchmarkId::new(format!("{ty}-simd"), len), |bencher| {
        bencher.iter(|| black_box(dot::<T, T, T>(black_box(a), black_box(b))))
    });
    group.bench_function(BenchmarkId::new(format!("{ty}-naive"), len), |bencher| {
        bencher.iter(|| black_box(naive_dot::<T, T, T>(black_box(a), black_box(b))))
    });
    group.bench_function(BenchmarkId::new(format!("{ty}-lanes"), len), |bencher| {
        bencher.iter(|| black_box(dot_lanes::<T>(black_box(a), black_box(b))))
    });
}

fn bench_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot");
    for len in LENS {
        group.throughput(Throughput::Elements(len as u64));
        bench_type(
            &mut group,
            "f64",
            len,
            &f64_samples(len, 0.0),
            &f64_samples(len, 1.0),
        );
        bench_type(
            &mut group,
            "f32",
            len,
            &f32_samples(len, 0.0),
            &f32_samples(len, 1.0),
        );
        bench_type(
            &mut group,
            "i32",
            len,
            &i32_samples(len, 0),
            &i32_samples(len, 3),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_dot);
criterion_main!(benches);
