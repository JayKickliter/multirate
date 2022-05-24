use crate::{collections::queue::Queue, math::dotprod::dot};
use core::ops::Mul;
use num_traits::Zero;

/// A stateful, streaming, FIR filter.
pub struct FIR<H, X> {
    /// Filter taps stored in reverse-order.
    h: Box<[H]>,
    /// Previous input samples.
    history: Queue<X>,
}

impl<H, X, O> FIR<H, X>
where
    H: Mul<X, Output = O>,
    X: Copy,
{
    pub fn with_taps(taps: &[H]) -> Self
    where
        H: Clone,
        X: Zero,
    {
        Self {
            h: taps.iter().rev().cloned().collect(),
            history: Queue::with_capacity(taps.len()),
        }
    }

    pub fn exec(&mut self, x: X) -> O
    where
        H: Copy,
        O: Zero,
    {
        self.history.push(x);
        let (x0, x1) = self.history.as_slices();
        let (h0, h1) = self.h.split_at(x0.len());
        dot(h0, x0) + dot(h1, x1)
    }

    pub fn h(&self) -> &[H] {
        &self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fir_create() {
        let taps = &[1.0_f32, 2.0, 3.0];
        let filt: FIR<f32, f32> = FIR::with_taps(taps);
        assert_eq!(filt.h(), &[3.0_f32, 2.0, 1.0]);
    }

    #[test]
    fn test_fir_exec() {
        let taps = &[1, 2, 3];
        let mut filt: FIR<i32, i32> = FIR::with_taps(taps);
        let xs = &[1, 0, 0, 0];
        let ys: Vec<i32> = xs.iter().map(|x| filt.exec(*x)).collect();
        assert_eq!(ys, [1, 2, 3, 0]);
    }
}
