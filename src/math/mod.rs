pub mod dotprod;

use itertools::Itertools;
use std::ops::Sub;

pub fn diff<'a, I, Elem>(into_iter: I) -> impl Iterator<Item = Elem> + 'a
where
    I: IntoIterator<Item = &'a Elem>,
    <I as IntoIterator>::IntoIter: 'a,
    Elem: 'a + Clone + Sub<Output = Elem>,
{
    into_iter
        .into_iter()
        .tuple_windows()
        .map(|(l, r)| r.clone() - l.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let xs: Vec<i32> = (0..5).into_iter().collect();
        let dxs: Vec<i32> = diff(&xs).collect();
        assert!(itertools::equal(diff(&xs), [1, 1, 1, 1]));
    }
}
