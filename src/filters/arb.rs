use crate::{
    collections::{pfb::PFB, queue::Queue},
    math::diff,
};
use num_traits::Zero;
use std::{num::NonZeroUsize, ops::Sub};

/// A FIR filter capable of resamping at non-rational rates.
#[derive(Clone, Debug)]
pub struct Arb<H, X> {
    /// Base filterbank.
    pfb: PFB<H>,
    /// Derivative filterbank.
    dpfb: PFB<H>,
    /// Previous input samples.
    history: Queue<X>,
    /// Resample rate.
    resamp_rate: f64,
    /// Current phase.
    phase: f64,
}

impl<H, X> Arb<H, X>
where
    H: Clone + Zero + Sub<Output = H>,
    X: Zero,
{
    /// Create a new resampler with provied taps split into `n` subfilters.
    pub fn with_taps(h: &[H], n: NonZeroUsize, resamp_rate: f64) -> Self {
        let pfb = PFB::with_taps(h, n);
        let dh: Vec<H> = diff(h).collect();
        let dpfb = PFB::with_taps(&dh, n);
        let history = Queue::with_capacity(pfb.dims().1);
        Self {
            pfb,
            dpfb,
            history,
            resamp_rate,
            phase: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_taps() {
        let h: Vec<i32> = (0..12).into_iter().collect();
        let _resampler: Arb<_, i32> = Arb::with_taps(&h, NonZeroUsize::new(4).unwrap(), 0.5);
    }
}
