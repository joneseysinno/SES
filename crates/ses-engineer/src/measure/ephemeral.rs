use ses_core::Ephemeral;

impl Ephemeral for super::Measure {
    const DERIVED_FROM: &'static [&'static str] = &["quantity", "unit-registry"];
}
