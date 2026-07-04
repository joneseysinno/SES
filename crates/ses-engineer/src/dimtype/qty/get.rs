use super::{DimType, Qty};

impl<D: DimType, S> Qty<D, S> {
    /// Borrow the inner scalar.
    pub fn get(&self) -> &S {
        &self.value
    }
}
