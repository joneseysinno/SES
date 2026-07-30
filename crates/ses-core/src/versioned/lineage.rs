use super::Root;

mod via {
    use core::marker::PhantomData;

    use super::super::Versioned;
    use super::Root;

    /// Witness that `T` belongs to a version lineage chain.
    pub trait Via<T: Versioned> {}

    /// Genesis generation: predecessor is [`Root`].
    pub enum Genesis {}

    impl<T> Via<T> for Genesis where T: Versioned<Supersedes = Root> {}

    /// Successor generation: decodes from `Prev` via `From`.
    pub struct FromPrev<Prev>(PhantomData<Prev>);

    impl<T, Prev> Via<T> for FromPrev<Prev>
    where
        T: Versioned<Supersedes = Prev>,
        T: From<Prev>,
        Prev: super::Lineage,
    {
    }
}

pub use via::{FromPrev, Genesis, Via};

use super::Versioned;

/// Member of a version lineage chain (Vocabulary §1.2).
///
/// Types declare [`Versioned::LineageVia`] as [`Genesis`] or [`FromPrev`].
/// Missing `From` decoders fail at compile time (see `tests/ui/missing_lineage_decoder.rs`).
pub trait Lineage: Versioned
where
    Self::LineageVia: Via<Self>,
{
}

impl<T> Lineage for T
where
    T: Versioned,
    T::LineageVia: Via<T>,
{
}
