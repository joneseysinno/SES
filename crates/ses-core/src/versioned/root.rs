/// Sealed sentinel: lineage chains terminate here (Vocabulary §1.2).
///
/// Genesis schema types set [`Versioned::Supersedes`](super::Versioned::Supersedes)
/// to this empty enum rather than referencing a predecessor.
pub enum Root {}
