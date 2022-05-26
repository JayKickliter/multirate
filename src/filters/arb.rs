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
    _pfb: PFB<H>,
    /// Derivative filterbank.
    _dpfb: PFB<H>,
    /// Previous input samples.
    _history: Queue<X>,
    /// Resample rate.
    _resamp_rate: f64,
    /// Current phase.
    _phase: f64,
}

impl<H, X> Arb<H, X>
where
    H: Clone + Zero + Sub<Output = H>,
    X: Zero,
{
    /// Create a new resampler with provied taps split into `n` subfilters.
    pub fn with_taps(h: &[H], n: NonZeroUsize, _resamp_rate: f64) -> Self {
        let _pfb = PFB::with_taps(h, n);
        let _dh: Vec<H> = diff(h).collect();
        let _dpfb = PFB::with_taps(&_dh, n);
        let _history = Queue::with_capacity(_pfb.dims().1);
        Self {
            _pfb,
            _dpfb,
            _history,
            _resamp_rate,
            _phase: 0.0,
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
