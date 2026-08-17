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
}
