use core::marker::PhantomData;

use super::DimType;

/// Scalar tagged with a compile-time dimension.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Qty<D: DimType, S> {
    pub(crate) value: S,
    pub(crate) _dim: PhantomData<D>,
}

mod add;
mod div;
mod get;
mod into_inner;
mod map;
mod mul;
mod new;
mod sub;
