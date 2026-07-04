use crate::error::EngineerError;
use crate::tower::Stress;

/// Dimension-gated sqrt in psi context (ses-provision-dsl §3).
pub fn sqrt_psi(value: Stress) -> Result<Stress, EngineerError> {
    Ok(Stress::new(value.into_inner().sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rational::Rational;
    use crate::tower::{Stress, lift};

    #[test]
    fn sqrt_four_psi() {
        let stress = Stress::new(lift(Rational::from_int(4)));
        let root = sqrt_psi(stress).unwrap();
        assert_eq!(root.into_inner(), lift(Rational::from_int(2)));
    }
}
