/// Witness for a certified tower comparison (Vocabulary §1.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertifiedBy {
    /// Comparison method identifier.
    pub method: &'static str,
    /// Enclosure width numerator (0 when exact).
    pub width_num: i64,
    /// Enclosure width denominator (1 when exact).
    pub width_den: i64,
}

impl CertifiedBy {
    /// Exact rational comparison — zero enclosure width.
    pub const fn exact_rational() -> Self {
        Self {
            method: "exact_rational",
            width_num: 0,
            width_den: 1,
        }
    }

    /// Interval comparison with a certified rational width.
    pub const fn interval(width_num: i64, width_den: i64) -> Self {
        Self {
            method: "interval",
            width_num,
            width_den,
        }
    }
}
