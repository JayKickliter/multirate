use num_traits::Zero;
use std::num::NonZeroUsize;

/// A polyphase filterbank capable of returning any arbitrary sub-filter.
#[repr(transparent)]
pub struct PFB<H>(Box<[Box<[H]>]>);

impl<H: Clone + Zero> PFB<H> {
    pub fn with_taps(h: &[H], n: NonZeroUsize) -> Self {
        Self(decompose(h, n))
    }
}

impl<H> PFB<H> {
    pub fn dims(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

/// Decompose $ \mathbf{h} $ into $ n $ subfilters, or filter
/// "phases".
///
/// # References
///
/// [Polyphase Filters](http://www.ws.binghamton.edu/fowler/fowler%20personal%20page/EE521_files/IV-05%20Polyphase%20FIlters%20Revised.pdf)
///
/// [LiquidDSP](https://github.com/jgaeddert/liquid-dsp/blob/b10acc5ab86480ccff4a0743702a082c4fafb4b7/src/filter/src/firpfb.proto.c)
pub fn decompose<H>(h: &[H], n: NonZeroUsize) -> Box<[Box<[H]>]>
where
    H: Clone + Zero,
{
    let n = n.into();
    let padding = match h.len() % n {
        0 => 0,
        r => n - r,
    };

    (0..n)
        .into_iter()
        .map(|ni| {
            h.iter()
                .chain(std::iter::repeat(&H::zero()).take(padding))
                .cloned()
                .skip(ni)
                .step_by(n)
                .collect::<Box<[H]>>()
        })
        .collect::<Box<[Box<[H]>]>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decompose() {
        let h: Vec<i32> = (0..11).into_iter().collect();
        let subfilters = decompose(&h, NonZeroUsize::new(4).unwrap());
        assert_eq!(
            subfilters,
            vec![
                vec![0, 4, 8].into_boxed_slice(),
                vec![1, 5, 9].into_boxed_slice(),
                vec![2, 6, 10].into_boxed_slice(),
                vec![3, 7, 0].into_boxed_slice()
            ]
            .into_boxed_slice()
        );
    }
}
