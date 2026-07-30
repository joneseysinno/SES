//! Shared error vocabulary (ses-core-build-plan §8).

use core::fmt;

use crate::dim::Dim;

/// Runtime dimension algebra errors (Vocabulary §1.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimError {
    /// An exponent operation would overflow `i8`.
    ExponentOverflow {
        /// Which basis axis overflowed.
        axis: &'static str,
        /// Operand exponent on that axis.
        lhs: i8,
        /// Other operand exponent on that axis.
        rhs: i8,
    },
    /// Dimensions differ where compatibility was required.
    Mismatch {
        /// Expected dimension.
        expected: Dim,
        /// Found dimension.
        found: Dim,
    },
}

impl fmt::Display for DimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExponentOverflow { axis, lhs, rhs } => {
                write!(f, "dimension exponent overflow on {axis}: {lhs} and {rhs}")
            }
            Self::Mismatch { expected, found } => {
                write!(f, "dimensions differ: expected {expected}, found {found}")
            }
        }
    }
}

impl std::error::Error for DimError {}

/// Inert rational representation errors (Vocabulary §1.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RationalError {
    /// Denominator was zero.
    ZeroDenominator,
    /// Intermediate computation exceeded `i64` range.
    Overflow,
    /// Decimal string could not be parsed exactly.
    ParseError {
        /// Original input fragment.
        input: String,
        /// Human-readable reason.
        reason: &'static str,
    },
}

impl fmt::Display for RationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDenominator => f.write_str("denominator must not be zero"),
            Self::Overflow => f.write_str("rational value overflowed i64 range"),
            Self::ParseError { input, reason } => {
                write!(f, "cannot parse rational from \"{input}\": {reason}")
            }
        }
    }
}

impl std::error::Error for RationalError {}

/// Authored-text parse failure kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoredErrorKind {
    /// Input was empty or whitespace only.
    EmptyInput,
    /// Unexpected character at offset.
    UnexpectedChar,
    /// Number could not be parsed.
    BadNumber,
    /// Fraction had zero denominator or bad form.
    BadFraction,
    /// Two measures appeared without an operator.
    AdjacentMeasures,
    /// Trailing separator (e.g. `3'-`).
    TrailingSeparator,
    /// Unit symbol before its quantity (`ft 24`).
    UnitBeforeQuantity,
    /// Ambiguous compact form (`3x8`).
    AmbiguousProduct,
}

impl fmt::Display for AuthoredErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => f.write_str("empty input"),
            Self::UnexpectedChar => f.write_str("unexpected character"),
            Self::BadNumber => f.write_str("invalid number"),
            Self::BadFraction => f.write_str("invalid fraction"),
            Self::AdjacentMeasures => f.write_str("adjacent measures need an operator"),
            Self::TrailingSeparator => f.write_str("trailing separator"),
            Self::UnitBeforeQuantity => f.write_str("unit before quantity"),
            Self::AmbiguousProduct => f.write_str("ambiguous product (use spaces around x)"),
        }
    }
}

/// Authored-text parse error with byte offset for UI underlining.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredParseError {
    /// Byte offset into the input string.
    pub offset: usize,
    /// Error classification.
    pub kind: AuthoredErrorKind,
}

impl fmt::Display for AuthoredParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at offset {}: {}", self.offset, self.kind)
    }
}

impl std::error::Error for AuthoredParseError {}
