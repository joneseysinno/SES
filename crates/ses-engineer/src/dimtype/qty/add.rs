use core::ops::Add;

use super::{DimType, Qty};

impl<D, S> Add<Qty<D, S>> for Qty<D, S>
where
    D: DimType,
    S: Add<Output = S>,
{
    type Output = Qty<D, S>;

    fn add(self, rhs: Qty<D, S>) -> Self::Output {
        Qty::new(self.value + rhs.value)
    }
}
