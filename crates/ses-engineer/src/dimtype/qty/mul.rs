use core::ops::Mul;

use crate::dimtype::{DimMul, DimType, Qty};

impl<Da, Db, S> Mul<Qty<Db, S>> for Qty<Da, S>
where
    Da: DimMul<Db>,
    S: Mul<Output = S>,
    Db: DimType,
{
    type Output = Qty<<Da as DimMul<Db>>::Out, S>;

    fn mul(self, rhs: Qty<Db, S>) -> Self::Output {
        Qty::new(self.value * rhs.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimtype::markers::{L1, L2};

    #[test]
    fn qty_length_mul() {
        let a: Qty<L1, i64> = Qty::new(24);
        let b: Qty<L1, i64> = Qty::new(2);
        let area: Qty<L2, i64> = a * b;
        assert_eq!(area.into_inner(), 48);
    }
}
