use core::ops::Sub;

use super::{DimType, Qty};

impl<D, S> Sub<Qty<D, S>> for Qty<D, S>
where
    D: DimType,
    S: Sub<Output = S>,
{
    type Output = Qty<D, S>;

    fn sub(self, rhs: Qty<D, S>) -> Self::Output {
        Qty::new(self.value - rhs.value)
    }
}
