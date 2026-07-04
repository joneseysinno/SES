/// A computed value bundled with its justification (Vocabulary §1.3).
///
/// Decisions must carry the evidence that produced them — ordering verdicts,
/// policy applications, and certified comparisons all use this shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decided<T, J> {
    /// The decided outcome.
    pub value: T,
    /// Justification witness (method, policy, enclosure width, etc.).
    pub justification: J,
}

impl<T, J> Decided<T, J> {
    /// Constructs a decided value with its justification.
    pub const fn new(value: T, justification: J) -> Self {
        Self {
            value,
            justification,
        }
    }
}
