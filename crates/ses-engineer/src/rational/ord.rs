use core::cmp::Ordering;

use super::Rational;

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.num == other.num && self.den == other.den {
            return Ordering::Equal;
        }
        let lhs = (self.num as i128) * (other.den as i128);
        let rhs = (other.num as i128) * (self.den as i128);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord_cross_multiply() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(2, 5).unwrap();
        assert!(a < b);
    }
}
