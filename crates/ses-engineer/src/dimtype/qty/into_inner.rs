use super::{DimType, Qty};

impl<D: DimType, S> Qty<D, S> {
    /// Unwrap the inner scalar.
    pub fn into_inner(self) -> S {
        self.value
    }
}
