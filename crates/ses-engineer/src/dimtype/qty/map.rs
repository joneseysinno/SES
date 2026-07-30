use super::{DimType, Qty};

impl<D: DimType, S> Qty<D, S> {
    /// Map the scalar while preserving the dimension tag.
    pub fn map<T>(self, f: impl FnOnce(S) -> T) -> Qty<D, T> {
        Qty::new(f(self.value))
    }
}
