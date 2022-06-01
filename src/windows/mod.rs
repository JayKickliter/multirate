use std::f64::consts::PI as PI64;
use std::marker::PhantomData;

pub trait Window<H>: Sized {
    fn tap(&self, n: usize, ns: usize) -> H;
    fn taps(self, ns: usize) -> Iter<Self, H> {
        Iter {
            window: self,
            n: 0,
            ns,
            _tap: PhantomData,
        }
    }
}

/// $$
/// w\[n\] = 0.53836−0.46164 \cos \left( \frac{2\pi n}{N-1} \right)
/// $$
#[derive(Copy, Clone, Debug)]
pub struct Hamming;

impl<H> Window<H> for Hamming
where
    H: From<f64>,
{
    fn tap(&self, n: usize, ns: usize) -> H {
        (0.54 - 0.46 * f64::cos((2.0 * PI64 * n as f64) / (ns as f64 - 1.0))).into()
    }
}

/// $$
/// w\[n\] = 0.5−0.5 \cos \left( \frac{2\pi n}{N-1} \right)
/// $$
#[derive(Copy, Clone, Debug)]
pub struct Hann;

impl<H> Window<H> for Hann
where
    H: From<f64>,
{
    fn tap(&self, n: usize, ns: usize) -> H {
        (0.5 - 0.5 * f64::cos((2.0 * PI64 * n as f64) / (ns as f64 - 1.0))).into()
    }
}

#[doc(hidden)]
pub struct Iter<W, H> {
    window: W,
    n: usize,
    ns: usize,
    _tap: PhantomData<H>,
}

impl<W, H> Iterator for Iter<W, H>
where
    W: Window<H>,
{
    type Item = H;

    fn next(&mut self) -> Option<H> {
        if self.n < self.ns {
            let res = Some(self.window.tap(self.n, self.ns));
            self.n += 1;
            res
        } else {
            None
        }
    }
}

#[cfg(test)]
mod trests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_hamming() {
        // Generated from Julia using `DSP.hamming(9)`
        let expected = [
            0.08000000000000002,
            0.21473088065418822,
            0.54,
            0.865269119345812,
            1.0,
            0.865269119345812,
            0.54,
            0.21473088065418822,
            0.08000000000000002,
        ];
        let taps: Vec<f64> = Hamming.taps(9).collect();
        assert_relative_eq!(taps.as_slice(), expected.as_slice());
    }
}
