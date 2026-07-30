use super::{DimType, Qty};

impl<D: DimType, S> Qty<D, S> {
    /// Wrap a scalar with dimension `D`.
    pub fn new(value: S) -> Self {
        Self {
            value,
            _dim: core::marker::PhantomData,
        }
    }
}
