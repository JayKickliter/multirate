use crate::collections::{pfb::PFB, queue::Queue};
use num_traits::Zero;
use std::num::NonZeroUsize;

/// A FIR filter capable of resamping at non-rational rates.
pub struct Arb<H, X> {
    /// Base filterbank.
    pfb: PFB<H>,
    // /// Derivative filterbank.
    // dpfb: PFB<H>,
    /// Previous input samples.
    history: Queue<X>,
    /// Resample rate.
    resamp_rate: f64,
    /// Current phase.
    phase: f64,
}

impl<H, X> Arb<H, X>
where
    H: Clone + Zero,
    X: Zero,
{
    /// Create a new resampler with provied taps split into `n` subfilters.
    pub fn with_taps(h: &[H], n: NonZeroUsize, resamp_rate: f64) -> Self {
        let pfb = PFB::with_taps(h, n);
        let history = Queue::with_capacity(pfb.dims().1);
        Self {
            pfb,
            history,
            resamp_rate,
            phase: 0.0,
        }
    }
}
