///! This module contains naive and optmimized dot-product routines.
use multiversion::multiversion;
use num_traits::Zero;

// Computing the XOR of two byte slices, `lhs` & `rhs`.
// `lhs` is mutated in-place with the result
#[multiversion]
#[clone(target = "[arm,aarch64]+neon")]
#[clone(target = "x86+sse")]
#[clone(target = "[x86|x86_64]+avx")]
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
