pub mod dotprod;

pub trait Diff {
    type Output;
    fn diff(self) -> Self::Output;
}
