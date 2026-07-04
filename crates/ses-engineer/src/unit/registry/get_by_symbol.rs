use super::UnitRegistry;

/// Normalize parser unit symbols to registry symbols (Vocabulary §1.3).
fn normalize_symbol(symbol: &str) -> String {
    let trimmed = symbol.trim();
    match trimmed {
        "\"" | "″" => "in".to_string(),
        "'" | "′" => "ft".to_string(),
        "lbf·in" | "lbf.in" => "lbf-in".to_string(),
        "lbf·ft" | "lbf.ft" => "lbf-ft".to_string(),
        "kip·in" | "kip.in" => "kip-in".to_string(),
        "kip·ft" | "kip.ft" => "kip-ft".to_string(),
        "Δ°F" | "°F" | "deg F" => "degF".to_string(),
        _ => trimmed.to_string(),
    }
}

impl UnitRegistry {
    /// Resolve a unit symbol string (Vocabulary §1.3).
    pub fn get_by_symbol(&self, symbol: &str) -> Option<&super::UnitEntry> {
        let normalized = normalize_symbol(symbol);
        self.entries.iter().find(|entry| entry.symbol == normalized)
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::imperial_seed;

    #[test]
    fn inch_aliases() {
        let reg = imperial_seed();
        assert_eq!(
            reg.get_by_symbol("\"").map(|e| e.symbol.as_str()),
            Some("in")
        );
        assert_eq!(
            reg.get_by_symbol("in").map(|e| e.symbol.as_str()),
            Some("in")
        );
    }

    #[test]
    fn foot_aliases() {
        let reg = imperial_seed();
        assert_eq!(
            reg.get_by_symbol("'").map(|e| e.symbol.as_str()),
            Some("ft")
        );
        assert_eq!(
            reg.get_by_symbol("ft").map(|e| e.symbol.as_str()),
            Some("ft")
        );
    }
}
