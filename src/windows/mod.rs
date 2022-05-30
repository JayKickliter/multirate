use num_traits::Float;
use std::f64::consts::PI as PI64;
use std::marker::PhantomData;

pub trait Window<H> {
    fn tap(&self, n: usize, ns: usize) -> H;
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
        (0.53836 - 0.46164 * f64::cos((2.0 * PI64 * n as f64) / (ns as f64 - 1.0))).into()
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
        unimplemented!()
    }
}
