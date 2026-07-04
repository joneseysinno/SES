use core::ops::Div;

use crate::dimtype::{DimDiv, DimType, Qty};

impl<Da, Db, S> Div<Qty<Db, S>> for Qty<Da, S>
where
    Da: DimDiv<Db>,
    S: Div<Output = S>,
    Db: DimType,
{
    type Output = Qty<<Da as DimDiv<Db>>::Out, S>;

    fn div(self, rhs: Qty<Db, S>) -> Self::Output {
        Qty::new(self.value / rhs.value)
    }
}
