use crate::{
    collections::{pfb::PFB, queue::Queue},
    math::diff,
};
use num_traits::Zero;
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};
use std::ops::Sub;

/// A FIR filter capable of resamping at non-rational rates.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Arb<H, X = H> {
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
    ///
    /// # Panics
    ///
    /// Will panic if `n < 2`
    pub fn with_taps(h: &[H], n: usize, _resamp_rate: f64) -> Self {
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
        let _resampler: Arb<i32> = Arb::with_taps(&h, 4, 0.5);
    }
}
