//! Naive and optimized dot-product routines.

use multiversion::multiversion;
use num_traits::Zero;

/// Computes the dot-product of two slices `a` and `b`.
#[multiversion(targets = "simd")]
pub fn dot<A, B, Prod>(a: &[A], b: &[B]) -> Prod
where
    A: Copy + std::ops::Mul<B, Output = Prod>,
    B: Copy,
    Prod: Zero,
{
    a.iter()
        .zip(b.iter())
        .fold(Prod::zero(), |acc: Prod, (a, b)| acc + (*a * *b))
}

pub fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
    dot(a, b)
}

/// Independent accumulators, enough to fill several SIMD registers.
const LANES: usize = 4;

/// Computes the dot-product of `a` and `b` with lane-split accumulators.
///
/// Splitting the sum into [LANES] independent chains lets the compiler
/// vectorize an operation that is otherwise blocked by the
/// non-associativity of floating-point addition. Summation order differs
/// from [dot], so results differ in the low bits.
#[multiversion(targets = "simd")]
pub fn dot_lanes<T>(a: &[T], b: &[T]) -> T
where
    T: Copy + Zero + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    let len = a.len().min(b.len());
    let (a, b) = (&a[..len], &b[..len]);
    let mut acc = [T::zero(); LANES];
    let mut a_chunks = a.chunks_exact(LANES);
    let mut b_chunks = b.chunks_exact(LANES);

    for (a, b) in a_chunks.by_ref().zip(b_chunks.by_ref()) {
        for lane in 0..LANES {
            acc[lane] += a[lane] * b[lane];
        }
    }

    let mut sum = T::zero();
    for lane in acc {
        sum += lane;
    }
    for (a, b) in a_chunks.remainder().iter().zip(b_chunks.remainder().iter()) {
        sum += *a * *b;
    }
    sum
}

pub fn dot_lanes_f32(a: &[f32], b: &[f32]) -> f32 {
    dot_lanes(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = [
            1.0_f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
        ];
        let b = [
            1.0_f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
        ];
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len() as f32, dot(&a, &b));
    }

    #[test]
    fn test_dot_lanes_matches_dot() {
        for len in [0, 1, LANES - 1, LANES, LANES + 1, 100] {
            let a: Vec<f32> = (0..len).map(|n| (n as f32 * 0.1).sin()).collect();
            let b: Vec<f32> = (0..len).map(|n| (n as f32 * 0.1).cos()).collect();
            let expected: f32 = dot(&a, &b);
            approx::assert_relative_eq!(expected, dot_lanes(&a, &b), epsilon = 1e-5);
        }
    }

    #[test]
    fn test_dot_lanes_ragged_lengths() {
        let a: Vec<i32> = (0..40).collect();
        let b: Vec<i32> = (0..25).collect();
        assert_eq!(dot::<i32, i32, i32>(&a, &b), dot_lanes(&a, &b));
        assert_eq!(dot::<i32, i32, i32>(&b, &a), dot_lanes(&b, &a));
    }
}
