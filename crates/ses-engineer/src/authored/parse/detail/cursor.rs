//! Shared scanner state for authored parsing (Vocabulary §1.3).

/// Byte offset into the authored source string.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    /// Current parse offset.
    pub offset: usize,
}

#[allow(dead_code)]
impl Cursor {
    /// Create a cursor at `offset`.
    #[allow(dead_code)]
    pub fn new(offset: usize) -> Self {
        Self { offset }
    }
}
